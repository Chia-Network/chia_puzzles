#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::programs::{CAT_PUZZLE, SINGLETON_TOP_LAYER};
    #[test]
    fn puzzle_hashes() {
        assert!(CAT_PUZZLE.len() == 1672);
        assert!(SINGLETON_TOP_LAYER.len() == 1168);
    }
}
