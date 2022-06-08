use std::{
    cell::Cell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt,
    hash::{Hash, Hasher},
};

fn main() {
    let s = Vertex::new("s");
    let t = Vertex::new("t");
    let x = Vertex::new("x");
    let y = Vertex::new("y");
    let z = Vertex::new("z");

    // A map from vertices to their adjacent vertices including costs
    let mut adjacency_list = HashMap::new();
    adjacency_list.insert(&s, vec![(&t, 10), (&y, 5)]);
    adjacency_list.insert(&t, vec![(&y, 2), (&x, 1)]);
    adjacency_list.insert(&x, vec![(&z, 4)]);
    adjacency_list.insert(&y, vec![(&t, 3), (&x, 9), (&z, 2)]);
    adjacency_list.insert(&z, vec![(&s, 7), (&x, 6)]);

    dijkstra(&s, &adjacency_list);

    adjacency_list.keys().for_each(|v| println!("{}", v));
}

fn dijkstra(start: &Vertex<'_>, adjacency_list: &HashMap<&Vertex<'_>, Vec<(&Vertex<'_>, usize)>>) {
    start.distance.set(0);

    // Fill the binary heap, vertices with the smallest distance go first
    let mut to_visit = BinaryHeap::new();
    adjacency_list.keys().for_each(|v| to_visit.push(*v));

    // We visit the vertices with the smallest distance first, this is
    // what makes Dijkstra a greedy algorithm
    while let Some(v) = to_visit.pop() {
        if let Some(neighbors) = adjacency_list.get(v) {
            for (n, cost) in neighbors {
                let new_distance = v.distance.get() + cost;
                if new_distance < n.distance.get() {
                    n.distance.set(new_distance);
                }
            }
            // When changing a vertex' distance, the BinaryHeap doesn't
            // update the position of the vertex.
            // That's why we create a new heap with the right order.
            let mut new_heap = BinaryHeap::new();
            to_visit.iter().for_each(|x| new_heap.push(*x));
            to_visit = new_heap;
        }
    }
}

#[derive(Eq)]
struct Vertex<'a> {
    name: &'a str,
    distance: Cell<usize>,
}

impl<'a> Vertex<'a> {
    fn new(name: &'a str) -> Vertex<'a> {
        Vertex {
            name,
            distance: Cell::new(usize::max_value()),
        }
    }
}
impl<'a> Hash for Vertex<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// Since this Vertex will be put in a priority queue where the vertices
/// with the *smallest* distance should be processed first, `cmp`
/// returns GT if self.distance().get() < other.distance().get()
impl<'a> Ord for Vertex<'a> {
    fn cmp(&self, other: &Vertex<'a>) -> Ordering {
        other.distance.get().cmp(&self.distance.get())
    }
}
impl<'a> PartialOrd for Vertex<'a> {
    fn partial_cmp(&self, other: &Vertex<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> PartialEq for Vertex<'a> {
    fn eq(&self, other: &Vertex<'a>) -> bool {
        self.name == other.name
    }
}

impl<'a> fmt::Display for Vertex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, distance: {}", self.name, self.distance.get())
    }
}
