#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

// performs Dijsktra 's algorithm on the given graph from the given start
// the graph is a positively-weighted undirected graph
//
// returns a map that for each reachable vertex associates the distance and the predecessor
// since the start has no predecessor but is reachable, map[start] will be None
pub fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
    start: &V,
) -> BTreeMap<V, Option<(V, E)>> {
    let mut ans = BTreeMap::new();
    let mut prio = BinaryHeap::new();

    // start is the special case that doesn't have a predecessor
    ans.insert(*start, None);

    for (new, weight) in &graph[start] {
        ans.insert(*new, Some((*start, *weight)));
        prio.push(Reverse((*weight, new, start)));
    }

    while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
        match ans[new] {
            // what we popped is what is in ans, we'll compute it
            Some((p, d)) if p == *prev && d == dist_new => {}
            // otherwise it's not interesting
            _ => continue,
        }

        for (next, weight) in &graph[new] {
            match ans.get(next) {
                // if ans[next] is a lower dist than the alternative one, we do nothing
                Some(Some((_, dist_next))) if dist_new + *weight >= *dist_next => {}
                // if ans[next] is None then next is start and so the distance won't be changed,
                // it won't be added again in prio
                Some(None) => {}
                // the new path is shorter, either new was not in ans or it was farther
                _ => {
                    ans.insert(*next, Some((*new, *weight + dist_new)));
                    prio.push(Reverse((*weight + dist_new, next, new)));
                }
            }
        }
    }

    ans
}
