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

    'process_todo: while let Some(line) = todo.pop_front() {
        for x in todo.iter().chain(&done) {
            if *x >= line {
                let mut key = line.finite.clone();
                key.push(line.infinite);
                useless.insert(key);
                continue 'process_todo;
            }
        }

        println!("found: {}", show_line(&line));

        // Remove lines obsoleted by newly found ones
        {
            let mut i = 0;
            while i < todo.len() {
                if line >= todo[i] {
                    println!(
                        "removed from todo: {} < {}",
                        show_line(&todo[i]),
                        show_line(&line)
                    );
                    todo.swap_remove_back(i);
                } else {
                    i += 1;
                }
            }
        }
        {
            let mut i = 0;
            while i < done.len() {
                if line >= done[i] {
                    println!(
                        "removed from done: {} < {}",
                        show_line(&done[i]),
                        show_line(&line)
                    );
                    done.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }

        done.push(line.clone());

        for d in &done {
            let mut candidates = vec![];
            'outer: for mut p in d.combinations(&line) {
                p.finite.sort();
                let mut key = p.finite.clone();
                key.push(p.infinite);
                if useless.contains(&key) || d >= &p || &line >= &p {
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
            todo.extend(candidates);
        }
    }

    let mut strings = done.iter().map(show_line).collect::<Vec<_>>();
    strings.sort();
    println!("{}", strings.join("\n"));
}
