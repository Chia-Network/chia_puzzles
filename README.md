# Chia Puzzles

This is a collection of the standard Chia puzzles.
These are the puzzles which are deployed on chain and therefore cannot change.

This repository tracks the source code, the compiled hex, and a hash of the compiled hex to ensure continuity.

All puzzles are kept in the `puzzles` folder as both a `.clsp` and `.clsp.hex` file.

The Python and Rust bindings are created by running `generate_chialisp_constants.py`

## Adding Bindings for a New Puzzle

1. Add the `.clsp` and `.clsp.hex` files into the `programs` folder.
2. Add the puzzle name, path, and shatree hash into the list in `generate_chialisp_constants.py`.
3. Run `generate_chialisp_constants.py`.
4. Commit your changes, including to the outputted `programs.py` and `programs.rs`.

# Testing

This project is managed with `poetry` for Python and `cargo` for Rust.

To run the Python tests:

```
python -m venv venv

pip install poetry
. ./venv/bin/activate
poetry install --extras=dev
pytest chia_puzzles_py/tests
```

If you're on Windows activate the venv with `. venv\Scripts\activate` instead

To run the Rust tests:

```
cargo test --all
```
