#[cfg(test)]
mod tests {
    use crate::programs::{CAT_PUZZLE_V2, SINGLETON_TOP_LAYER};
    #[test]
    fn puzzle_hashes() {
        assert!(CAT_PUZZLE_V2.len() == 1672);
        assert!(SINGLETON_TOP_LAYER.len() == 1168);
    }
}
