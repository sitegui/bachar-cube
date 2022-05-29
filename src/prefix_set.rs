use crate::OuterPiece;
use std::collections::{BTreeSet, HashSet};

const TOTAL_BYTES: usize = 25;
const PREFIX_BYTES: usize = 3;

#[derive(Debug, Clone)]
pub struct PrefixSet {
    partitions: Vec<HashSet<[u8; TOTAL_BYTES - PREFIX_BYTES]>>,
}

impl PrefixSet {
    pub fn new() -> Self {
        let max_tag = dbg!(OuterPiece::MAX_TAG) as usize + 1;
        PrefixSet {
            partitions: (0..max_tag * max_tag * max_tag)
                .map(|_| HashSet::new())
                .collect(),
        }
    }

    pub fn insert(&mut self, bytes: [u8; TOTAL_BYTES]) -> bool {
        let partition =
            bytes[0] as usize * OuterPiece::MAX_TAG as usize * OuterPiece::MAX_TAG as usize
                + bytes[1] as usize * OuterPiece::MAX_TAG as usize
                + bytes[2] as usize;
        let suffix = bytes[PREFIX_BYTES..].try_into().unwrap();
        self.partitions[partition].insert(suffix)
    }

    pub fn len(&self) -> usize {
        self.partitions.iter().map(|p| p.len()).sum()
    }
}
