#![feature(const_evaluatable_checked)]
#![feature(const_generics)]

mod bitarray;
mod line;

use bitarray::BitArray;
use itertools::Itertools;
use line::Line;
use std::collections::{BTreeSet, VecDeque};
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

    match degree {
        2 => active_side::<1, 2>(passive, alphabet),
        3 => active_side::<1, 3>(passive, alphabet),
        4 => active_side::<1, 4>(passive, alphabet),
        5 => active_side::<1, 5>(passive, alphabet),
        _ => println!("Didn't compile version for degree {}", degree),
    }
}

fn active_side<const C: usize, const D: usize>(passive: Vec<Vec<BitArray<C>>>, alphabet: Vec<u8>)
where
    [(); D - 1]: Sized,
{
    let show_set = |set: &BitArray<C>| {
        String::from_utf8(
            alphabet
                .iter()
                .enumerate()
                .filter(|(i, _)| set.get(*i))
                .map(|(_, x)| *x)
                .collect(),
        )
        .unwrap()
    };

    let show_line = |line: &Line<C, D>| {
        let mut tmp = line.0.iter().map(show_set).collect::<Vec<_>>();
        tmp.sort();
        tmp.join(" ")
    };

    let mut todo: VecDeque<Line<C, D>> = passive
        .into_iter()
        .map(|line| Line(line.try_into().unwrap()))
        .collect();
    let mut done: Vec<Line<C, D>> = vec![];

    while let Some(line) = todo.pop_front() {
        done.push(line.clone());

        let perms: Vec<Line<C, D>> = line
            .0
            .iter()
            .cloned()
            .permutations(D)
            .map(|x| Line(x.try_into().unwrap()))
            .collect();

        let mut i = 0;
        while i < done.len() {
            let mut next_i = i + 1;
            for p in &perms {
                'new_lines: for mut new in done[i].combine_with(&p) {
                    for x in done.iter().chain(&todo).rev() {
                        if new.maximize_with(x) {
                            continue 'new_lines;
                        }
                    }

                    println!("found: {} via {}", show_line(&new), show_line(&line));

                    // Remove lines obsoleted by newly found ones
                    {
                        let mut i = 0;
                        while i < todo.len() {
                            if todo[i] < new {
                                println!(
                                    "removed from todo: {} < {}",
                                    show_line(&todo[i]),
                                    show_line(&new)
                                );
                                todo.swap_remove_back(i);
                            } else {
                                i += 1;
                            }
                        }
                    }

                    let mut write = 0;
                    for j in 0..done.len() {
                        if done[j] < new {
                            println!(
                                "removed from done: {} < {}",
                                show_line(&done[j]),
                                show_line(&new)
                            );
                            if j < next_i {
                                next_i -= 1;
                            }
                        } else {
                            // TODO Unnecessary clone here.
                            done[write] = done[j].clone();
                            write += 1;
                        }
                    }
                    done.truncate(write);

                    todo.push_back(new);
                }
            }
            i = next_i;
        }
    }

    let mut strings = done.iter().map(show_line).collect::<Vec<_>>();
    strings.sort();
    println!("{}", strings.join("\n"));
}
