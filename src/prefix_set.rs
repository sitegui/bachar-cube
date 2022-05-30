use parking_lot::Mutex;
use std::collections::HashSet;
use std::mem;
use std::mem::MaybeUninit;

/// Represent the set of all possible [`u64`].
#[derive(Debug)]
pub struct PrefixSet {
    children: [Mutex<HashSet<u64>>; 256],
}

impl PrefixSet {
    pub fn new() -> Self {
        // Taken from https://doc.rust-lang.org/std/mem/union.MaybeUninit.html
        let children = {
            // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
            // safe because the type we are claiming to have initialized here is a
            // bunch of `MaybeUninit`s, which do not require initialization.
            let mut children: [MaybeUninit<Mutex<HashSet<u64>>>; 256] =
                unsafe { MaybeUninit::uninit().assume_init() };

            // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
            // assignment instead of `ptr::write` does not cause the old
            // uninitialized value to be dropped. Also if there is a panic during
            // this loop, we have a memory leak, but there is no memory safety
            // issue.
            for elem in &mut children[..] {
                elem.write(Mutex::new(HashSet::new()));
            }

            // Everything is initialized. Transmute the array to the
            // initialized type.
            unsafe { mem::transmute::<_, [Mutex<HashSet<u64>>; 256]>(children) }
        };

        PrefixSet { children }
    }

    pub fn insert(&self, value: u64) -> bool {
        let first_byte = value >> 56;
        self.children[first_byte as usize]
            .lock()
            .insert(value ^ (first_byte << 56))
    }

    pub fn len(&self) -> usize {
        self.children.iter().map(|child| child.lock().len()).sum()
    }
}
