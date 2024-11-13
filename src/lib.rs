mod tree_hash;

pub use tree_hash::*;

use chia_puzzles_macro::include_puzzles;
use hex_literal::hex;

include_puzzles!("puzzles");
