from __future__ import annotations

import dataclasses
import hashlib
import io
import json
import pathlib
import tempfile
import traceback
import typing

import typing_extensions
from clvm.CLVMObject import CLVMStorage
from clvm.serialize import sexp_from_stream
from clvm.SExp import SExp
from clvm_tools_rs import compile_clvm

here = pathlib.Path(__file__).parent.resolve()
root = here.parent
cache_path = root.joinpath(".chia_cache", "manage_clvm.json")

# Suffixes and path definitions
clvm_suffix = ".clvm"
clsp_suffix = ".clsp"
hex_suffix = ".clsp.hex"
all_suffixes = {"clsp": clsp_suffix, "hex": hex_suffix, "clvm": clvm_suffix}
top_levels = {"chia"}
hashes_path = root.joinpath("../puzzle_hashes.json")
std_libraries = root.joinpath("../puzzles")

ValueType = typing.Union[bytes, CLVMStorage]
ValueStackType = typing.List[ValueType]
Op = typing.Callable[[ValueStackType, "OpStackType", typing.Set[bytes]], None]
OpStackType = typing.List[Op]


def std_hash(
    b: typing.Union[bytes, typing.SupportsBytes], skip_bytes_conversion: bool = False
) -> bytes:
    """
    The standard hash used in many places.
    """
    if skip_bytes_conversion:
        # casting for hinting based on above overloads constraining the type
        return hashlib.sha256(typing.cast(bytes, b)).digest()
    else:
        return hashlib.sha256(bytes(b)).digest()


def sha256_treehash(
    sexp: CLVMStorage, precalculated: typing.Optional[typing.Set[bytes]] = None
) -> bytes:
    """
    Hash values in `precalculated` are presumed to have been hashed already.
    """

    if precalculated is None:
        precalculated = set()

    def handle_sexp(
        sexp_stack: ValueStackType,
        op_stack: OpStackType,
        precalculated: typing.Set[bytes],
    ) -> None:
        # just trusting it is right, otherwise we get an attribute error
        sexp: SExp = sexp_stack.pop()  # type: ignore[assignment]
        if sexp.pair:
            p0, p1 = sexp.pair
            sexp_stack.append(p0)
            sexp_stack.append(p1)
            op_stack.append(handle_pair)
            op_stack.append(handle_sexp)
            op_stack.append(roll)
            op_stack.append(handle_sexp)
        else:
            # not a pair, so an atom
            atom: bytes = sexp.atom  # type: ignore[assignment]
            if atom in precalculated:
                r = atom
            else:
                r = std_hash(b"\1" + atom)
            sexp_stack.append(r)

    def handle_pair(
        sexp_stack: ValueStackType,
        op_stack: OpStackType,
        precalculated: typing.Set[bytes],
    ) -> None:
        # just trusting it is right, otherwise we get a type error
        p0: bytes = sexp_stack.pop()  # type: ignore[assignment]
        p1: bytes = sexp_stack.pop()  # type: ignore[assignment]
        sexp_stack.append(std_hash(b"\2" + p0 + p1))

    def roll(
        sexp_stack: ValueStackType,
        op_stack: OpStackType,
        precalculated: typing.Set[bytes],
    ) -> None:
        p0 = sexp_stack.pop()
        p1 = sexp_stack.pop()
        sexp_stack.append(p0)
        sexp_stack.append(p1)

    sexp_stack: ValueStackType = [sexp]
    op_stack: typing.List[Op] = [handle_sexp]
    while len(op_stack) > 0:
        op = op_stack.pop()
        op(sexp_stack, op_stack, precalculated)

    result: bytes = sexp_stack[0]  # type: ignore[assignment]
    return result


class ManageClvmError(Exception):
    pass


class CacheEntry(typing.TypedDict):
    clsp: str
    hex: str
    hash: str


class Cache(typing_extensions.TypedDict):
    entries: CacheEntries
    version: CacheVersion


CacheEntries = dict[str, CacheEntry]
CacheVersion = list[int]
current_cache_version: CacheVersion = [1]


class CacheVersionError(ManageClvmError):
    pass


def create_empty_cache() -> Cache:
    return {"entries": {}, "version": current_cache_version}


def load_cache(file: typing.IO[str]) -> Cache:
    loaded_cache = typing.cast(Cache, json.load(file))
    loaded_version = loaded_cache.get("version")
    if loaded_version != current_cache_version:
        raise CacheVersionError("Cache has wrong version.")
    return loaded_cache


def dump_cache(cache: Cache, file: typing.IO[str]) -> None:
    json.dump(cache, file, indent=2)


def generate_hash_bytes(hex_bytes: bytes) -> bytes:
    cleaned_blob = bytes.fromhex(hex_bytes.decode("utf-8"))
    serialized_hash = sha256_treehash(SExp.to(cleaned_blob))
    return serialized_hash


@dataclasses.dataclass(frozen=True)
class ClvmPaths:
    clvm: pathlib.Path
    hex: pathlib.Path
    hash: str
    missing_files: list[str]

    @classmethod
    def from_clvm(cls, clvm: pathlib.Path, hash_dict: dict[str, str] = {}) -> ClvmPaths:
        stem_filename = clvm.name[: -len(clsp_suffix)]
        hex_path = clvm.with_name(stem_filename + hex_suffix)
        missing_files = []
        if not hex_path.exists():
            missing_files.append(str(hex_path))
        return cls(
            clvm=clvm, hex=hex_path, hash=stem_filename, missing_files=missing_files
        )


@dataclasses.dataclass(frozen=True)
class ClvmBytes:
    hex: bytes
    hash: bytes

    @classmethod
    def from_clvm_paths(
        cls, paths: ClvmPaths, hash_dict: dict[str, str] = {}
    ) -> ClvmBytes:
        hex_bytes = paths.hex.read_bytes()
        return cls(hex=hex_bytes, hash=generate_hash_bytes(hex_bytes=hex_bytes))

    @classmethod
    def from_hex_bytes(cls, hex_bytes: bytes) -> ClvmBytes:
        return cls(
            hex=hex_bytes,
            hash=generate_hash_bytes(hex_bytes=hex_bytes),
        )


def find_stems(
    top_levels: set[str], suffixes: typing.Mapping[str, str] = all_suffixes
) -> dict[str, set[pathlib.Path]]:
    return {
        name: {
            path.with_name(path.name[: -len(suffix)])
            for top_level in top_levels
            for path in root.joinpath(top_level).rglob(f"**/*{suffix}")
        }
        for name, suffix in suffixes.items()
    }


def create_cache_entry(
    reference_paths: ClvmPaths, reference_bytes: ClvmBytes
) -> CacheEntry:
    source_bytes = reference_paths.clvm.read_bytes()

    clvm_hasher = hashlib.sha256()
    clvm_hasher.update(source_bytes)

    hex_hasher = hashlib.sha256()
    hex_hasher.update(reference_bytes.hex)

    hash_hasher = hashlib.sha256()
    hash_hasher.update(reference_bytes.hash)

    return {
        "clsp": clvm_hasher.hexdigest(),
        "hex": hex_hasher.hexdigest(),
        "hash": hash_hasher.hexdigest(),
    }


def check_cache(cache: Cache, HASHES: dict[str, str]) -> bool:
    overall_fail = False
    try:
        cache_entries = cache["entries"]
        found_stems = find_stems(top_levels)
        for stem_path in sorted(found_stems["clsp"]):
            reference_paths = ClvmPaths.from_clvm(
                clvm=stem_path.with_name(stem_path.name + clsp_suffix), hash_dict=HASHES
            )
            if reference_paths.missing_files:
                continue
            reference_bytes = ClvmBytes.from_clvm_paths(
                paths=reference_paths, hash_dict=HASHES
            )
            new_cache_entry = create_cache_entry(
                reference_paths=reference_paths, reference_bytes=reference_bytes
            )
            cache_entries[str(stem_path)] = new_cache_entry
        return not overall_fail
    except Exception as e:
        traceback.print_exc()
        return False
