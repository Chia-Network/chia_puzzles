use std::{fs, path::PathBuf};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, LitStr};
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

    #[error("missing directory: {0}")]
    MissingDir(String),

    #[error("missing source file: {0}")]
    MissingSource(String),

    #[error("missing hex file: {0}")]
    MissingHex(String),

    #[error("invalid entry: {0}")]
    InvalidEntry(String),

    #[error("invalid file type: {0}")]
    InvalidFileType(String),
}

pub fn include_puzzles(input: TokenStream) -> Result<TokenStream, Error> {
    let puzzles_dir: LitStr = parse2(input)?;

    let mut puzzles = Vec::new();
    crawl_dir(puzzles_dir.value().into(), &mut puzzles)?;

    let mut items = Vec::new();

    for Puzzle { name, source, hex } in puzzles {
        items.push(quote! {
            /// #source
            pub const #name: &str = #hex;
        });
    }

    Ok(quote! {})
}

fn crawl_dir(path: PathBuf, puzzles: &mut Vec<Puzzle>) -> Result<(), Error> {
    let path_string = path.to_str().expect("invalid path").to_string();

    for entry in fs::read_dir(path).map_err(|_| Error::MissingDir(path_string.clone()))? {
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

        let source_path = format!("{name}.clsp");
        let source =
            fs::read_to_string(&source_path).map_err(|_| Error::MissingSource(source_path))?;

        let hex_path = entry.path().to_str().expect("invalid path").to_string();
        let hex = fs::read_to_string(&hex_path).map_err(|_| Error::MissingHex(hex_path))?;

        puzzles.push(Puzzle {
            name: name.to_case(Case::ScreamingSnake),
            source: source.trim().to_string(),
            hex: hex.trim().to_string(),
        })
    }

    Ok(())
}
