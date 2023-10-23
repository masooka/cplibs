use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    iter::{self, Product, Sum},
    ops::{Add, AddAssign},
};

pub fn costs<V, Es, Vs, Ws, WsI, W>(start: V, mut neighbors: Es, mut costs: Ws, mut cost: WsI) -> Ws
where
    V: Copy + Ord,
    Es: FnMut(V) -> Vs,
    Vs: IntoIterator<Item = (V, W)>,
    WsI: FnMut(&mut Ws, V) -> &mut W,
    W: Copy + Ord + Add<Output = W> + Sum,
{
    *cost(&mut costs, start) = iter::empty().sum();
    let queue = &mut BinaryHeap::from(vec![(Reverse(iter::empty().sum()), start)]);
    while let Some((Reverse(current_cost), current_node)) = queue.pop() {
        if *cost(&mut costs, current_node) < current_cost {
            continue;
        }
        for (next_node, cost_delta) in neighbors(current_node) {
            let next_cost = current_cost + cost_delta;
            if next_cost < *cost(&mut costs, next_node) {
                *cost(&mut costs, next_node) = next_cost;
                queue.push((Reverse(next_cost), next_node));
            }
        }
    }
    costs
}

pub fn costs_and_counts<V, Es, Vs, Ws, WsI, W, Cs, CsI, C>(
    start: V,
    mut neighbors: Es,
    mut costs: Ws,
    mut cost: WsI,
    mut counts: Cs,
    mut count: CsI,
) -> (Ws, Cs)
where
    V: Copy + Ord,
    Es: FnMut(V) -> Vs,
    Vs: IntoIterator<Item = (V, W)>,
    WsI: FnMut(&mut Ws, V) -> &mut W,
    W: Copy + Ord + Add<Output = W> + Sum,
    CsI: FnMut(&mut Cs, V) -> &mut C,
    C: Copy + AddAssign + Product,
{
    *cost(&mut costs, start) = iter::empty().sum();
    *count(&mut counts, start) = iter::empty().product();
    let queue = &mut BinaryHeap::from(vec![(Reverse(iter::empty().sum()), start)]);
    while let Some((Reverse(current_cost), current_node)) = queue.pop() {
        if *cost(&mut costs, current_node) < current_cost {
            continue;
        }
        for (next_node, cost_delta) in neighbors(current_node) {
            let next_cost = current_cost + cost_delta;
            match next_cost.cmp(cost(&mut costs, next_node)) {
                Ordering::Less => {
                    *cost(&mut costs, next_node) = next_cost;
                    *count(&mut counts, next_node) = *count(&mut counts, current_node);
                    queue.push((Reverse(next_cost), next_node));
                }
                Ordering::Equal => {
                    let current_count = *count(&mut counts, current_node);
                    *count(&mut counts, next_node) += current_count;
                }
                Ordering::Greater => {}
            }
        }
    }
    (costs, counts)
}
