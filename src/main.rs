use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Please enter an input file as command line argument.");
    let passive: Vec<Vec<Vec<u8>>> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(|x| x.bytes().collect())
                .collect()
        })
        .collect();
    let degree = passive[0].len();

    match degree {
        2 => active_side::<2>(passive.into_iter().map(|x| x.try_into().unwrap()).collect()),
        4 => active_side::<4>(passive.into_iter().map(|x| x.try_into().unwrap()).collect()),
        5 => active_side::<5>(passive.into_iter().map(|x| x.try_into().unwrap()).collect()),
        _ => println!("Didn't compile version for degree {}", degree),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Status {
    Allowed { forever: bool },
    Forbidden,
}
use Status::*;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Line<const D: usize> {
    sets: [[Status; 256]; D],
}

impl<const D: usize> std::fmt::Display for Line<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for set in &self.sets {
            let bytes = (0..256)
                .filter(|i| set[*i] != Forbidden)
                .map(|x| x as u8)
                .collect::<Vec<_>>();
            write!(f, "{} ", String::from_utf8(bytes).unwrap())?
        }
        Ok(())
    }
}

fn active_side<const D: usize>(passive: Vec<[Vec<u8>; D]>) {
    let alphabet: Vec<u8> = passive
        .iter()
        .flatten()
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .cloned()
        .collect();

    let all_permutations: HashSet<[u8; D]> = passive
        .iter()
        .flat_map(|line| {
            line.iter()
                .cloned()
                .multi_cartesian_product()
                .flat_map(|x| x.into_iter().permutations(D))
        })
        .map(|x| x.try_into().unwrap())
        .collect();
    dbg!("perms done");

    let all_bad = vec![alphabet.clone(); D]
        .into_iter()
        .multi_cartesian_product()
        .map(|x| x.try_into().unwrap())
        .filter(|x: &[u8; D]| !all_permutations.contains(x))
        .collect::<Vec<_>>();
    dbg!("bad computed");

    let mut all_allowed = [Forbidden; 256];
    for x in alphabet {
        all_allowed[x as usize] = Allowed { forever: false };
    }

    println!(
        "{}",
        rec(
            Line {
                sets: [all_allowed; D]
            },
            all_bad.iter().cloned()
        )
        .unwrap()
    );
}

fn rec<const D: usize>(
    line: Line<D>,
    mut bads: impl Iterator<Item = [u8; D]> + Clone,
) -> Option<Line<D>> {
    if let Some(bad) = bads.next() {
        if bad
            .iter()
            .zip(&line.sets)
            .any(|(b, set)| set[*b as usize] == Forbidden)
        {
            rec(line, bads)
        } else {
            for (i, b) in bad.iter().enumerate() {
                if line.sets[i][*b as usize] != (Allowed { forever: true })
                    && line.sets[i].iter().filter(|x| **x != Forbidden).count() != 1
                {
                    let mut line = line.clone();
                    line.sets[i][*b as usize] = Forbidden;
                    for j in i + 1..D {
                        line.sets[j][bad[j] as usize] = Allowed { forever: true };
                    }

                    if let Some(res) = rec(line, bads.clone()) {
                        return Some(res);
                    }
                }
            }
            None
        }
    } else {
        Some(line)
    }
}
