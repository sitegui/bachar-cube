use crate::OuterPiece;
use parking_lot::Mutex;
use std::collections::HashSet;
use std::mem;
use std::mem::MaybeUninit;

/// Represent the set of all possible [`u64`].
#[derive(Debug)]
pub struct PrefixSet {
    children: [Mutex<Level6>; 256],
}

macro_rules! generate_intermediate_level {
    ($name:ident, $child:ident) => {
        #[derive(Debug)]
        struct $name {
            children: [Option<Box<$child>>; 256],
        }

        impl $name {
            fn new() -> Self {
                const INIT: Option<Box<$child>> = None;
                $name {
                    children: [INIT; 256],
                }
            }

            fn insert(&mut self, value: u64) -> bool {
                let last_byte = value & 0xFF;
                let child = self.children[last_byte as usize]
                    .get_or_insert_with(|| Box::new($child::new()));
                child.insert(value >> 8)
            }

            fn len(&self) -> usize {
                let mut total = 0;
                for child in &self.children {
                    if let Some(child) = child {
                        total += child.len();
                    }
                }
                total
            }
        }
    };
}

generate_intermediate_level! {Level6, Level5}
generate_intermediate_level! {Level5, Level4}
generate_intermediate_level! {Level4, Level3}
generate_intermediate_level! {Level3, Level2}
generate_intermediate_level! {Level2, Level1}

/// A set of `u16`, encoded as 65 536 bits, stored in 2 048 `u32`s.
///
/// The values 0 to 31 map to the first block. The value 0 maps to the least-significant bit of the
/// block.
#[derive(Debug, Clone, Copy)]
struct Level1 {
    blocks: [Block; NUM_BLOCKS],
}

type Block = u32;
const NUM_BLOCKS: usize = 256 * 256 / BLOCK_LEN;
const BLOCK_LEN: usize = 32;
const BLOCK_LEN_LOG2: u32 = 5;

impl PrefixSet {
    pub fn new() -> Self {
        let children = {
            // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
            // safe because the type we are claiming to have initialized here is a
            // bunch of `MaybeUninit`s, which do not require initialization.
            let mut children: [MaybeUninit<Mutex<Level6>>; 256] =
                unsafe { MaybeUninit::uninit().assume_init() };

            // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
            // assignment instead of `ptr::write` does not cause the old
            // uninitialized value to be dropped. Also if there is a panic during
            // this loop, we have a memory leak, but there is no memory safety
            // issue.
            for elem in &mut children[..] {
                elem.write(Mutex::new(Level6::new()));
            }

            // Everything is initialized. Transmute the array to the
            // initialized type.
            unsafe { mem::transmute::<_, [Mutex<Level6>; 256]>(children) }
        };

        PrefixSet { children }
    }

    pub fn insert(&self, value: u64) -> bool {
        let last_byte = value & 0xFF;
        self.children[last_byte as usize].lock().insert(value >> 8)
    }

    pub fn len(&self) -> usize {
        self.children.iter().map(|child| child.lock().len()).sum()
    }
}

impl Level1 {
    fn new() -> Self {
        Level1 {
            blocks: [0; NUM_BLOCKS],
        }
    }

    fn insert(&mut self, value: u64) -> bool {
        let block_index = value >> BLOCK_LEN_LOG2;
        let bit_index = value % BLOCK_LEN as u64;

        let block = &mut self.blocks[block_index as usize];
        let inserted = ((*block >> bit_index) & 0b1) == 0;
        *block |= 1 << bit_index;
        inserted
    }

    fn len(&self) -> usize {
        let mut total = 0;
        for block in self.blocks {
            total += block.count_ones();
        }
        total as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level3() {
        for n in 0..=1_000 {
            let mut set = Level1::new();
            assert!(!set.insert(n));
            assert!(set.insert(n));
        }
    }
}
