use crate::bitarray::{zero, BitArray};
use std::cmp::Ordering;

#[derive(Clone, Debug, Hash)]
pub struct Line<const C: usize, const DEGREE: usize>(pub [BitArray<C>; DEGREE]);

impl<const C: usize, const D: usize> Line<C, D>
where
    [(); D - 1]: Sized,
{
    /// Generates all useful combinations of two lines that don't require permuting them.
    pub fn combine_with(&self, other: &Line<C, D>) -> impl Iterator<Item = Line<C, D>> {
        let mut intersections = self.0.clone();
        for (me, other) in intersections.iter_mut().zip(&other.0) {
            *me = *me & *other;
        }

        let me = self.0.clone();
        let other = other.0.clone();

        (0..D).filter_map(move |i| {
            let union = me[i] | other[i];

            // If one side doesn't contribute anything to the union,
            // the result is just an inferior version of one of the lines
            if union == me[i] || union == other[i] {
                None
            } else {
                let mut x = intersections.clone();
                x[i] = union;
                if x.iter().any(|s| *s == zero()) {
                    None
                } else {
                    Some(Self(x))
                }
            }
        })
    }

    /// Returns whether this line is inferior or equal to the other line
    /// and adds symbols to this line if possible.
    pub fn maximize_with(&mut self, other: &Line<C, D>) -> bool {
        for my_index in 0..D {
            for other_index in 0..D {
                let mine = self.0[my_index];
                let others = other.0[other_index];
                if !(mine & others == others && mine != others)
                    && other.without(other_index) >= self.without(my_index)
                {
                    self.0[my_index] |= others;

                    // All other sets are inferior or equal.
                    // If this one isn't original either, this line is redundant.
                    if self.0[my_index] == others {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn without(&self, index: usize) -> Line<C, { D - 1 }> {
        let mut out = [zero(); D - 1];
        let mut j = 0;
        for i in 0..D {
            if i != index {
                out[j] = self.0[i];
                j += 1;
            }
        }
        Line(out)
    }
}

impl<const C: usize, const D: usize> Line<C, D> {
    /// The number of symbols allowed in total
    fn size(&self) -> usize {
        self.0.iter().map(|x| x.size()).sum()
    }
}

impl<const C: usize, const D: usize> Eq for Line<C, D> {}
impl<const C: usize, const D: usize> PartialEq<Self> for Line<C, D> {
    fn eq(&self, other: &Self) -> bool {
        let mut used = [false; D];
        'outer: for s in self.0 {
            for i in 0..D {
                if !used[i] && other.0[i] == s {
                    used[i] = true;
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }
}

impl<const C: usize, const D: usize> PartialOrd<Self> for Line<C, D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let own_size = self.size();
        let other_size = other.size();

        if own_size > other_size && self >= other {
            return Some(Ordering::Greater);
        }
        if own_size < other_size && other >= self {
            return Some(Ordering::Less);
        }
        if own_size == other_size && self == other {
            return Some(Ordering::Equal);
        }
        None
    }

    fn ge(&self, other: &Self) -> bool {
        let mut stack = vec![(0, [false; D])];

        while let Some((i, used)) = stack.pop() {
            for (j, o) in other.0.iter().enumerate() {
                if !used[j] && *o & self.0[i] == *o {
                    if i == D - 1 {
                        return true;
                    }
                    let mut used2 = used.clone();
                    used2[j] = true;
                    stack.push((i + 1, used2));
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::*;
    use std::convert::TryInto;

    impl Arbitrary for Line<1, 3> {
        fn arbitrary(g: &mut Gen) -> Self {
            Self(
                (0..3)
                    .map(|_| BitArray::arbitrary(g))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        }
    }

    #[quickcheck]
    fn maximize_identical(mut line: Line<1, 3>) -> bool {
        line.maximize_with(&line.clone())
    }
}
