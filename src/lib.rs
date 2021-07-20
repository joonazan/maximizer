#![feature(const_evaluatable_checked)]
#![feature(const_generics)]

pub mod bitarray;
mod line;
pub mod line_superiority;

use bitarray::BitArray;
use itertools::Itertools;
use line::Line;
use line_superiority::is_inferior_to;
use std::collections::VecDeque;
use std::convert::TryInto;

pub fn active_side<const C: usize, const D: usize>(
    passive: Vec<Vec<BitArray<C>>>,
    alphabet: Vec<u8>,
) where
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

            let mut candidates = vec![];
            'outer: for p in perms.iter().flat_map(|p| done[i].combine_with(p)) {
                for c in &candidates {
                    if *c >= p {
                        continue 'outer;
                    }
                }
                candidates.retain(|c| *c > p);
                candidates.push(p);
            }

            'new_lines: for mut new in candidates {
                for x in todo.iter().chain(&done) {
                    if new.maximize_with(x) {
                        continue 'new_lines;
                    }
                }

                println!("found: {} via {}", show_line(&new), show_line(&line));

                // Remove lines obsoleted by newly found ones
                {
                    let mut i = 0;
                    while i < todo.len() {
                        if is_inferior_to(&todo[i], &new) {
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

                let mut written = 0;
                for j in 0..done.len() {
                    if is_inferior_to(&done[j], &new) {
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
                        done[written] = done[j].clone();
                        written += 1;
                    }
                }
                done.truncate(written);

                todo.push_back(new);
            }
            i = next_i;
        }
    }

    let mut strings = done.iter().map(show_line).collect::<Vec<_>>();
    strings.sort();
    println!("{}", strings.join("\n"));
}
