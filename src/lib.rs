mod programs;

pub use programs::*;

#[cfg(test)]
mod tests {
    use crate::programs::{CAT_V2, SINGLETON_TOP_LAYER_V2};
    #[test]
    fn puzzle_hashes() {
        assert_eq!(CAT_V2.len(), 1672);
        assert_eq!(SINGLETON_TOP_LAYER_V2.len(), 967);
    }
}
