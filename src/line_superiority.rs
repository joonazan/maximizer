use crate::line::Line;
use std::collections::VecDeque;
use std::convert::TryInto;

pub fn is_inferior_to<const C: usize, const D: usize>(
    side_a: &Line<C, D>,
    side_b: &Line<C, D>,
) -> bool {
    let mut storage = vec![];
    let neighbors_a: [&[usize]; D] = (0..D)
        .map(|i| {
            let start = storage.len();
            for j in 0..D {
                if side_a.0[i] & side_b.0[j] == side_a.0[i] {
                    storage.push(j);
                }
            }
            start..storage.len()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| &storage[x])
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    maximum_matching_simple(&neighbors_a)
}

pub fn maximum_matching_simple<const D: usize>(neighbors_a: &[&[usize]; D]) -> bool {
    let mut stack = vec![(0, [false; D])];

    while let Some((i, used)) = stack.pop() {
        for n in neighbors_a[i] {
            if !used[*n] {
                if i == D - 1 {
                    return true;
                }
                let mut used2 = used.clone();
                used2[*n] = true;
                stack.push((i + 1, used2));
            }
        }
    }
    false
}

pub fn maximum_matching_hopcroft_karp<const D: usize>(neighbors_a: &[&[usize]; D]) -> bool {
    let mut pair_for_b = [None; D];
    let mut pair_for_a = [0; D];

    let mut starts: Vec<usize> = (0..D).collect();

    while !starts.is_empty() {
        let mut layer_a = [None; D];

        let mut todo = starts.clone();
        let mut next = vec![];
        let mut last_layer = false;

        for layer in 0.. {
            for a in &todo {
                if layer_a[*a] != None {
                    continue;
                }
                layer_a[*a] = Some(layer);

                for b in neighbors_a[*a] {
                    if let Some(ai2) = pair_for_b[*b] {
                        next.push(ai2);
                    } else {
                        last_layer = true;
                    }
                }
            }

            if last_layer {
                break;
            }

            if next.is_empty() {
                return false;
            }

            std::mem::swap(&mut todo, &mut next);
            next.clear();
        }

        let mut visited = [false; D];

        let mut i = 0;
        'outer: while i < starts.len() {
            let mut stack = vec![Try(starts[i])];
            let mut current_path = vec![];

            while let Some(f) = stack.pop() {
                match f {
                    Try(a) => {
                        if visited[a] {
                            continue;
                        }
                        visited[a] = true;
                        current_path.push(a);
                        stack.push(Backtrack);

                        for b in neighbors_a[a] {
                            if let Some(a2) = pair_for_b[*b] {
                                if layer_a[*b] == Some(layer_a[a].unwrap() + 1) {
                                    stack.push(Try(a2));
                                }
                            } else {
                                let mut b = *b;
                                while let Some(a) = current_path.pop() {
                                    pair_for_b[b] = Some(a);
                                    let old_b = b;
                                    b = pair_for_a[a];
                                    pair_for_a[a] = old_b;
                                }
                                starts.swap_remove(i);
                                continue 'outer;
                            }
                        }
                    }
                    Backtrack => {
                        current_path.pop();
                    }
                }
            }
            i += 1;
        }
    }

    true
}

enum StackFrame {
    Backtrack,
    Try(usize),
}
use StackFrame::*;

pub fn maximum_matching_push_relabel<const D: usize>(neighbors_a: &[&[usize]; D]) -> bool
where
    [(); 2 * D]: Sized,
{
    let mut height = [0; 2 * D];
    let mut excess = [0; 2 * D];
    let mut todo = VecDeque::new();

    for i in 0..D {
        excess[i] = 1;
        todo.push_back(i);
        excess[D + i] = -1;
    }

    // All edges represent a possibility for a unit flow
    let mut neighbors = neighbors_a
        .iter()
        .map(|x| x.iter().cloned().map(|x| x + D).collect())
        .chain((0..D).map(|_| vec![]))
        .collect::<Vec<Vec<usize>>>();

    while let Some(v) = todo.pop_front() {
        while excess[v] > 0 {
            let mut min_height = 4 * D;
            let mut index = 0;
            let mut v2 = 0;
            for (i, &v) in neighbors[v].iter().enumerate() {
                if height[v] < min_height {
                    min_height = height[v];
                    v2 = v;
                    index = i;
                }
            }

            if v < D && min_height > 2 * D {
                excess[v] = 0;
                height[v] = 2 * D + 1;
                break;
            }

            if height[v] <= min_height {
                height[v] = min_height + 1;
            }

            neighbors[v].swap_remove(index);
            neighbors[v2].push(v);

            excess[v] -= 1;
            excess[v2] += 1;
            if excess[v2] == 1 {
                todo.push_back(v2);
            }
        }
    }

    excess[D..].iter().all(|&x| x == 0)
}
