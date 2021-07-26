use std::{
    cmp::Ordering,
    ops::{BitAnd, BitOr, BitOrAssign, BitXor, Not},
};

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub struct BitArray<const CELLS: usize>([usize; CELLS]);

pub const fn cells_needed(bits: usize) -> usize {
    let width = std::mem::size_of::<usize>() * 8;
    (bits + width - 1) / width
}

pub const fn zero<const C: usize>() -> BitArray<C> {
    BitArray([0; C])
}

impl<const C: usize> BitArray<C> {
    pub fn size(&self) -> usize {
        self.0.iter().map(|x| x.count_ones()).sum::<u32>() as usize
    }
}

impl<const C: usize> Not for BitArray<C> {
    type Output = BitArray<C>;

    fn not(mut self) -> Self::Output {
        for i in 0..C {
            self.0[i] = !self.0[i];
        }
        self
    }
}

impl<const C: usize> BitOr for BitArray<C> {
    type Output = BitArray<C>;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        for i in 0..C {
            self.0[i] |= rhs.0[i];
        }
        self
    }
}

impl<const C: usize> BitOrAssign for BitArray<C> {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..C {
            self.0[i] |= rhs.0[i];
        }
    }
}

impl<const C: usize> BitAnd for BitArray<C> {
    type Output = BitArray<C>;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        for i in 0..C {
            self.0[i] &= rhs.0[i];
        }
        self
    }
}

impl<const C: usize> BitXor for BitArray<C> {
    type Output = BitArray<C>;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        for i in 0..C {
            self.0[i] ^= rhs.0[i];
        }
        self
    }
}

impl<const C: usize> BitArray<C> {
    pub fn get(&self, index: usize) -> bool {
        let width = std::mem::size_of::<usize>() * 8;
        (self.0[index / width] >> (index % width)) & 1 != 0
    }

    pub fn set(&mut self, index: usize) {
        let width = std::mem::size_of::<usize>() * 8;
        self.0[index / width] |= 1 << (index % width);
    }
}

impl<const C: usize> Eq for BitArray<C> {}

impl<const C: usize> PartialOrd for BitArray<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const C: usize> Ord for BitArray<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.0.iter().zip(other.0) {
            if *a > b {
                return Ordering::Greater;
            }
            if *a < b {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::*;
    impl Arbitrary for BitArray<1> {
        fn arbitrary(g: &mut Gen) -> Self {
            Self([usize::arbitrary(g); 1])
        }
    }
}
