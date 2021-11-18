use bitarray::BitArray;
use maximizer::{active_side, bitarray};
use std::collections::BTreeSet;
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

    let alphabet: Vec<u8> = passive
        .iter()
        .flat_map(|line| line.iter().flat_map(|set| set.iter()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .cloned()
        .collect();

    if bitarray::cells_needed(alphabet.len()) != 1 {
        panic!("Alphabets that big are currently unsupported.")
    }

    let passive: Vec<Vec<BitArray<1>>> = passive
        .iter()
        .map(|line| {
            line.iter()
                .map(|s| {
                    let mut out = bitarray::zero();
                    for x in s {
                        // SAFETY: always found because `alphabet` has every symbol in `passive`
                        out.set(alphabet.binary_search(x).unwrap());
                    }
                    out
                })
                .collect()
        })
        .collect();

    active_side::<1>(passive, alphabet)
}
