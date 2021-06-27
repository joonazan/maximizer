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
        3 => active_side::<3>(passive.into_iter().map(|x| x.try_into().unwrap()).collect()),
        4 => active_side::<4>(passive.into_iter().map(|x| x.try_into().unwrap()).collect()),
        5 => active_side::<5>(passive.into_iter().map(|x| x.try_into().unwrap()).collect()),
        _ => println!("Didn't compile version for degree {}", degree),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
enum Status {
    Allowed { forever: bool },
    Forbidden,
}
use Status::*;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Line<const D: usize> {
    sets: [Set; D],
    cardinality: usize,
}

impl<const D: usize> std::fmt::Display for Line<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for set in &self.sets {
            let bytes = (0..256)
                .filter(|i| set.contents[*i] != Forbidden)
                .map(|x| x as u8)
                .collect::<Vec<_>>();
            write!(f, "{} ", String::from_utf8(bytes).unwrap())?
        }
        Ok(())
    }
}

impl<const D: usize> Line<D> {
    fn contains(&self, other: &Line<D>) -> bool {
        if other.cardinality > self.cardinality {
            false
        } else {
            self.sets
                .iter()
                .zip(&other.sets)
                .all(|(mine, his)| mine.contains(his))
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Set {
    contents: [Status; 256],
    cardinality: usize,
}

impl std::ops::Index<u8> for Set {
    type Output = Status;

    fn index(&self, index: u8) -> &Self::Output {
        &self.contents[index as usize]
    }
}

impl std::ops::IndexMut<u8> for Set {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.contents[index as usize]
    }
}

impl Set {
    fn contains(&self, other: &Set) -> bool {
        if other.cardinality > self.cardinality {
            false
        } else {
            self.contents
                .iter()
                .zip(&other.contents)
                .all(|(mine, his)| *mine != Forbidden || *his == Forbidden)
        }
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

    let mut all_allowed = Set {
        contents: [Forbidden; 256],
        cardinality: alphabet.len(),
    };

    for x in &alphabet {
        all_allowed[*x] = Allowed { forever: false };
    }

    let all_line = Line {
        sets: (0..D)
            .map(|_| all_allowed.clone())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        cardinality: D * alphabet.len(),
    };

    for line in bfs(all_line, &all_bad) {
        println!("{}", line);
    }
}

fn find_one_line<const D: usize>(line: Line<D>, bads: &[[u8; D]]) -> Line<D> {
    if bads.is_empty() {
        return line;
    }

    struct Subtree<const D: usize> {
        line: Line<D>,
        bads_survived: usize,
        next_set: usize,
    }

    let mut stack = vec![Subtree {
        line,
        bads_survived: 0,
        next_set: 0,
    }];

    'outer: while let Some(mut state) = stack.pop() {
        loop {
            if state.next_set == D {
                continue 'outer;
            }

            let b = bads[state.bads_survived][state.next_set];
            let set = &state.line.sets[state.next_set];
            if set[b] != (Allowed { forever: true }) && set.cardinality != 1 {
                break;
            }
            state.next_set += 1;
        }

        stack.push(Subtree {
            line: state.line.clone(),
            bads_survived: state.bads_survived,
            next_set: state.next_set + 1,
        });

        state.line.sets[state.next_set][bads[state.bads_survived][state.next_set]] = Forbidden;
        state.line.sets[state.next_set].cardinality -= 1;
        state.line.cardinality -= 1;

        for j in state.next_set + 1..D {
            state.line.sets[j][bads[state.bads_survived][j]] = Allowed { forever: true };
        }
        state.bads_survived += 1;
        state.next_set = 0;

        if state.bads_survived == bads.len() {
            return state.line;
        }

        while bads[state.bads_survived]
            .iter()
            .zip(&state.line.sets)
            .any(|(b, set)| set[*b] == Forbidden)
        {
            state.bads_survived += 1;
            if state.bads_survived == bads.len() {
                return state.line;
            }
        }

        stack.push(state);
    }

    unreachable!()
}

fn bfs<const D: usize>(line: Line<D>, bads: &[[u8; D]]) -> Vec<Line<D>> {
    let mut lines = vec![line];

    for bad in bads {
        let mut new_lines = vec![];
        for line in lines {
            if bad
                .iter()
                .zip(&line.sets)
                .any(|(b, set)| set[*b] == Forbidden)
            {
                new_lines.push(line);
            } else {
                let adds = (0..D)
                    .filter(|i| {
                        line.sets[*i].cardinality > 1
                            && line.sets[*i][bad[*i]] != (Allowed { forever: true })
                    })
                    .map(|i| {
                        let mut line = line.clone();
                        line.sets[i][bad[i]] = Forbidden;
                        line.sets[i].cardinality -= 1;
                        line.cardinality -= 1;

                        for j in i + 1..D {
                            line.sets[j][bad[j]] = Allowed { forever: true };
                        }
                        line
                    })
                    .filter(|x| !new_lines.iter().any(|nl| nl.contains(x)))
                    .collect::<Vec<_>>();

                new_lines.extend(adds);
            }
        }
        lines = new_lines;
    }

    lines
}
