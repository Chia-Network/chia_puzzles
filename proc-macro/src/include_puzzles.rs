use std::{collections::HashMap, fs, io, path::PathBuf};

use indoc::formatdoc;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse2, Ident, LitStr};
use thiserror::Error;

struct Puzzle {
    name: String,
    source: String,
    hex: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("parse error: {0}")]
    Parse(#[from] syn::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("missing directory: {0}")]
    MissingDir(String),

    #[error("missing source file: {0}")]
    MissingSource(String),

    #[error("missing hex file: {0}")]
    MissingHex(String),

    #[error("missing puzzle hash for {0}")]
    MissingPuzzleHash(String),

    #[error("invalid entry: {0}")]
    InvalidEntry(String),

    #[error("invalid file type: {0}")]
    InvalidFileType(String),
}

pub fn include_puzzles(input: TokenStream) -> Result<TokenStream, Error> {
    let puzzles_dir: LitStr = parse2(input)?;
    let puzzles_dir: PathBuf = puzzles_dir.value().into();

    let puzzle_hashes_path = puzzles_dir.parent().unwrap().join("puzzle_hashes.json");
    let puzzle_hashes: HashMap<String, String> =
        serde_json::from_str(&fs::read_to_string(puzzle_hashes_path)?)?;

    let mut puzzles = Vec::new();
    crawl_dir(puzzles_dir, &mut puzzles)?;

    let mut items = Vec::new();

    for Puzzle { name, source, hex } in &puzzles {
        let snake_name = name.to_lowercase();
        let length = hex.len() / 2;

        let puzzle_hash = puzzle_hashes
            .get(&snake_name)
            .ok_or(Error::MissingPuzzleHash(snake_name.clone()))?;

        let hex_ident = Ident::new(name, Span::call_site());
        let puzzle_hash_ident = Ident::new(&format!("{}_PUZZLE_HASH", name), Span::call_site());

        let hex_doc = formatdoc!(
            "
            The hex representation of the compiled `{snake_name}` puzzle.
            You can also use [`{puzzle_hash_ident}`] to refer to the corresponding puzzle hash.

            # Source
            ```lisp
            \n{source}\n
            ```
            "
        );

        items.push(quote! {
            #[doc=#hex_doc]
            pub const #hex_ident: [u8; #length] = hex!(#hex);
        });

        let puzzle_hash_doc = formatdoc!(
            "
            The puzzle hash of the `{snake_name}` puzzle.
            You can also use [`{hex_ident}`] to refer to the full compiled puzzle reveal.
            "
        );

        items.push(quote! {
            #[doc=#puzzle_hash_doc]
            pub const #puzzle_hash_ident: TreeHash = TreeHash::new(hex!(#puzzle_hash));
        });
    }

    let mut tests = Vec::new();

    for Puzzle { name, .. } in puzzles {
        let test_name = Ident::new(&format!("test_{}", name.to_lowercase()), Span::call_site());
        let puzzle_hash_name = Ident::new(&format!("{}_PUZZLE_HASH", name), Span::call_site());
        let name = Ident::new(&name, Span::call_site());

        tests.push(quote! {
            #[test]
            fn #test_name() -> anyhow::Result<()> {
                let mut allocator = Allocator::new();

                let ptr = node_from_bytes(&mut allocator, &#name)?;
                let result = crate::test::tree_hash(&allocator, ptr);

                assert_eq!(result, #puzzle_hash_name);

                Ok(())
            }
        });
    }

    Ok(quote! {
        #( #items )*

        #[cfg(test)]
        mod tests {
            use super::*;

            use clvmr::{Allocator, serde::node_from_bytes};

            #( #tests )*
        }
    })
}

fn crawl_dir(path: PathBuf, puzzles: &mut Vec<Puzzle>) -> Result<(), Error> {
    let path_string = path.to_str().expect("invalid path").to_string();

    for entry in fs::read_dir(&path).map_err(|_| Error::MissingDir(path_string.clone()))? {
        let entry = entry.map_err(|_| Error::InvalidEntry(path_string.clone()))?;
        let file_type = entry
            .file_type()
            .map_err(|_| Error::InvalidFileType(path_string.clone()))?;

        if file_type.is_dir() {
            crawl_dir(entry.path(), puzzles)?;
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        let name = entry.file_name();

        let Some(name) = name
            .to_str()
            .expect("invalid file name")
            .strip_suffix(".clsp.hex")
        else {
            continue;
        };

        let source_path = path.join(format!("{name}.clsp"));
        let source_path = source_path.to_str().expect("invalid path").to_string();
        let source =
            fs::read_to_string(&source_path).map_err(|_| Error::MissingSource(source_path))?;

        let hex_path = entry.path().to_str().expect("invalid path").to_string();
        let hex = fs::read_to_string(&hex_path).map_err(|_| Error::MissingHex(hex_path))?;

        puzzles.push(Puzzle {
            name: name.to_uppercase(),
            source: source.trim().to_string(),
            hex: hex.trim().to_string(),
        })
    }

    Ok(())
}
