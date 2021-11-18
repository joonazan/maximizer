pub mod bitarray;
mod line;

use bitarray::BitArray;
use line::Line;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn active_side<const C: usize>(passive: Vec<Vec<BitArray<C>>>, alphabet: Vec<u8>) {
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

    let show_line = |line: &Line<C>| {
        let mut tmp = line.finite.iter().map(show_set).collect::<Vec<_>>();
        tmp.sort();
        tmp.push(show_set(&line.infinite));
        tmp.join(" ") + "*"
    };

    let mut todo: VecDeque<Line<C>> = passive
        .into_iter()
        .map(|mut line| Line {
            infinite: line.pop().unwrap(),
            finite: line,
        })
        .collect();
    let mut done: Vec<Line<C>> = vec![];

    let mut useless: HashSet<Vec<BitArray<C>>> = HashSet::new();

    while let Some(line) = todo.pop_front() {
        done.push(line.clone());

        let mut i = 0;
        while i < done.len() {
            let mut next_i = i + 1;

            let mut candidates = vec![];
            'outer: for mut p in done[i].combinations(&line) {
                p.finite.sort();
                let mut key = p.finite.clone();
                key.push(p.infinite);
                if useless.contains(&key) {
                    continue;
                }

                for c in &candidates {
                    if *c >= p {
                        useless.insert(key);
                        continue 'outer;
                    }
                }
                candidates.retain(|c| !(p >= *c));
                candidates.push(p);
            }

            'new_lines: for new in candidates {
                for x in todo.iter().chain(&done) {
                    if *x >= new {
                        let mut key = new.finite.clone();
                        key.push(new.infinite);
                        useless.insert(key);
                        continue 'new_lines;
                    }
                }

                println!("found: {} via {}", show_line(&new), show_line(&line));

                // Remove lines obsoleted by newly found ones
                {
                    let mut i = 0;
                    while i < todo.len() {
                        if new >= todo[i] {
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
                    if new >= done[j] {
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
