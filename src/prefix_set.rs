use itertools::Itertools;
use parking_lot::Mutex;
use std::collections::HashSet;
use std::mem;

/// Represent the set of all possible [`u64`].
#[derive(Debug)]
pub struct PrefixSet {
    children: Vec<Mutex<HashSet<u64>>>,
}

const PREFIX_BITS: u32 = 16;

impl PrefixSet {
    pub fn new() -> Self {
        PrefixSet {
            children: (0..2u32.pow(PREFIX_BITS))
                .map(|_| Mutex::new(HashSet::new()))
                .collect_vec(),
        }
    }

    pub fn insert(&self, value: u64) -> bool {
        let value = stir(value);
        let prefix = value >> (u64::BITS - PREFIX_BITS);
        let suffix = value ^ (prefix << (u64::BITS - PREFIX_BITS));
        self.children[prefix as usize].lock().insert(suffix)
    }

    pub fn len(&self) -> usize {
        self.children.iter().map(|child| child.lock().len()).sum()
    }
}

fn stir(v: u64) -> u64 {
    let [mut a, mut b, mut c, mut d] = unsafe { mem::transmute::<u64, [u16; 4]>(v) };

    for _ in 0..4 {
        // Slightly inspired by the core of SHA-1
        let new_a = (b ^ c).wrapping_add(d).wrapping_add(a.rotate_left(5));
        let new_b = a;
        let new_c = b.rotate_left(14);
        let new_d = c;

        a = new_a;
        b = new_b;
        c = new_c;
        d = new_d;
    }

    unsafe { mem::transmute::<[u16; 4], u64>([a, b, c, d]) }
}
