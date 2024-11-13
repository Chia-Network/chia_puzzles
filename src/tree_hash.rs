use std::{fmt, ops::Deref};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TreeHash([u8; 32]);

impl TreeHash {
    pub const fn new(hash: [u8; 32]) -> Self {
        Self(hash)
    }

    pub const fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl fmt::Debug for TreeHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TreeHash({self})")
    }
}

impl fmt::Display for TreeHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl From<[u8; 32]> for TreeHash {
    fn from(hash: [u8; 32]) -> Self {
        Self::new(hash)
    }
}

impl From<TreeHash> for [u8; 32] {
    fn from(hash: TreeHash) -> [u8; 32] {
        hash.0
    }
}

impl AsRef<[u8]> for TreeHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for TreeHash {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    use clvmr::{sha2::Sha256, Allocator, NodePtr, SExp};

    enum TreeOp {
        SExp(NodePtr),
        Cons,
    }

    fn tree_hash_atom(bytes: &[u8]) -> TreeHash {
        let mut sha256 = Sha256::new();
        sha256.update([1]);
        sha256.update(bytes);
        TreeHash::new(sha256.finalize())
    }

    fn tree_hash_pair(first: TreeHash, rest: TreeHash) -> TreeHash {
        let mut sha256 = Sha256::new();
        sha256.update([2]);
        sha256.update(first);
        sha256.update(rest);
        TreeHash::new(sha256.finalize())
    }

    pub fn tree_hash(a: &Allocator, node: NodePtr) -> TreeHash {
        let mut hashes = Vec::new();
        let mut ops = vec![TreeOp::SExp(node)];

        while let Some(op) = ops.pop() {
            match op {
                TreeOp::SExp(node) => match a.sexp(node) {
                    SExp::Atom => {
                        hashes.push(tree_hash_atom(a.atom(node).as_ref()));
                    }
                    SExp::Pair(left, right) => {
                        ops.push(TreeOp::Cons);
                        ops.push(TreeOp::SExp(left));
                        ops.push(TreeOp::SExp(right));
                    }
                },
                TreeOp::Cons => {
                    let first = hashes.pop().unwrap();
                    let rest = hashes.pop().unwrap();
                    hashes.push(tree_hash_pair(first, rest));
                }
            }
        }

        assert!(hashes.len() == 1);
        hashes[0]
    }
}
