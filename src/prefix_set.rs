use crate::OuterPiece;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct PrefixSet {
    partitions: Vec<BTreeSet<[u8; 23]>>,
}

impl PrefixSet {
    pub fn new() -> Self {
        let max_tag = OuterPiece::MAX_TAG as usize + 1;
        PrefixSet {
            partitions: (0..max_tag * max_tag).map(|_| BTreeSet::new()).collect(),
        }
    }

    pub fn insert(&mut self, bytes: [u8; 25]) -> bool {
        let partition = bytes[0] as usize * OuterPiece::MAX_TAG as usize + bytes[1] as usize;
        let suffix = bytes[2..].try_into().unwrap();
        self.partitions[partition].insert(suffix)
    }

    pub fn len(&self) -> usize {
        self.partitions.iter().map(|p| p.len()).sum()
    }
}
