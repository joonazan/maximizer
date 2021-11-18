use crate::bitarray::{zero, BitArray};
use std::cmp::Ordering;

#[derive(Clone, Debug, Hash)]
pub struct Line<const C: usize> {
    pub finite: Vec<BitArray<C>>,
    pub infinite: BitArray<C>,
}

struct Combinations<'a, const C: usize> {
    a: &'a Line<C>,
    b: &'a Line<C>,
    available: Vec<bool>,
    pairs: Vec<(Line<C>, Line<C>)>,
}

impl<const C: usize> Iterator for Combinations<'_, C> {
    type Item = Line<C>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        todo!()
    }
}

impl<const C: usize> Line<C> {
    /// Generates all combinations of two lines
    pub fn combinations(&self, other: &Line<C>) -> impl Iterator<Item = Line<C>> {
        std::iter::once(self.clone())
    }
}

impl<const C: usize> Eq for Line<C> {}
impl<const C: usize> PartialEq<Self> for Line<C> {
    fn eq(&self, other: &Self) -> bool {
        if self.infinite != other.infinite {
            return false;
        }
        // infinite is known to be the same
        let infinite = self.infinite;
        let mut used = vec![false; other.finite.len()];
        'outer: for s in &self.finite {
            if *s == infinite {
                continue;
            }
            for i in 0..other.finite.len() {
                if !used[i] && other.finite[i] == *s {
                    used[i] = true;
                    continue 'outer;
                }
            }
            return false;
        }

        for (i, o) in other.finite.iter().enumerate() {
            if !used[i] && *o != infinite {
                return false;
            }
        }
        true
    }
}

impl<const C: usize> PartialOrd<Self> for Line<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self >= other {
            return Some(Ordering::Greater);
        }
        if other >= self {
            return Some(Ordering::Less);
        }
        if self == other {
            return Some(Ordering::Equal);
        }
        None
    }

    fn ge(&self, other: &Self) -> bool {
        if !set_ge(&self.infinite, &other.infinite) {
            return false;
        }

        let (mandatory, optional): (Vec<BitArray<C>>, Vec<BitArray<C>>) = self
            .finite
            .iter()
            .partition(|s| !set_ge(s, &other.infinite));
        let mut ctx = GeCtx {
            used_other: vec![false; other.finite.len()],
            optional,
            other_mandatory: other
                .finite
                .iter()
                .enumerate()
                .filter(|(_, s)| !set_ge(&self.infinite, s))
                .map(|(i, _)| i)
                .collect::<Vec<_>>(),
            other_finite: &other.finite,
        };

        ctx.pair_self(mandatory.iter())
    }
}

struct GeCtx<'a, const C: usize> {
    used_other: Vec<bool>,
    optional: Vec<BitArray<C>>,
    other_mandatory: Vec<usize>,
    other_finite: &'a [BitArray<C>],
}

impl<const C: usize> GeCtx<'_, C> {
    fn pair_self<'a>(&mut self, mut todo: impl Iterator<Item = &'a BitArray<C>> + Clone) -> bool {
        if let Some(s) = todo.next() {
            for (i, o) in self.other_finite.iter().enumerate() {
                if !self.used_other[i] && set_ge(&s, o) {
                    self.used_other[i] = true;
                    if self.pair_self(todo.clone()) {
                        return true;
                    }
                    self.used_other[i] = false;
                }
            }
            false
        } else {
            let mut optional_used = vec![false; self.optional.len()];
            self.pair_other(self.other_mandatory.iter().cloned(), &mut optional_used)
        }
    }

    fn pair_other(
        &self,
        mut todo: impl Iterator<Item = usize> + Clone,
        optional_used: &mut [bool],
    ) -> bool {
        if let Some(j) = todo.next() {
            if !self.used_other[j] {
                for (i, s) in self.optional.iter().enumerate() {
                    if !optional_used[i] && set_ge(s, &self.other_finite[j]) {
                        optional_used[i] = true;
                        if self.pair_other(todo.clone(), optional_used) {
                            return true;
                        }
                        optional_used[i] = false;
                    }
                }
                false
            } else {
                self.pair_other(todo, optional_used)
            }
        } else {
            true
        }
    }
}

fn set_ge<const C: usize>(a: &BitArray<C>, b: &BitArray<C>) -> bool {
    *a & *b == *b
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::*;
    use std::convert::TryInto;

    impl Arbitrary for Line<1> {
        fn arbitrary(g: &mut Gen) -> Self {
            Self {
                finite: (0..3).map(|_| BitArray::arbitrary(g)).collect::<Vec<_>>(),
                infinite: BitArray::arbitrary(g),
            }
        }
    }

    #[derive(Clone, Debug)]
    struct GeTest {
        big: Line<1>,
        small: Line<1>,
    }

    impl Arbitrary for GeTest {
        fn arbitrary(g: &mut Gen) -> Self {
            fn mask(g: &mut Gen) -> BitArray<1> {
                let mut m = !zero();
                m.set(usize::arbitrary(g) % (std::mem::size_of::<usize>() * 8));
                m
            }
            let big = Line::arbitrary(g);
            let mut smallf: Vec<BitArray<1>> = big.finite.iter().map(|x| *x & mask(g)).collect();
            for i in (0..smallf.len()).rev() {
                smallf.swap(i, usize::arbitrary(g) % (i + 1))
            }
            let small = Line {
                finite: smallf,
                infinite: big.infinite & mask(g),
            };
            Self { big, small }
        }
    }

    #[quickcheck]
    fn eq_self(line: Line<1>) -> bool {
        line == line
    }

    #[quickcheck]
    fn line_ge(test: GeTest) -> bool {
        test.big >= test.small
    }

    #[quickcheck]
    fn line_gt_excludes_lt(a: Line<1>, b: Line<1>) -> bool {
        let ge = a >= b;
        let le = b >= a;
        let eq = a == b;
        !eq && !(ge && le) || eq && ge && le
    }
}
