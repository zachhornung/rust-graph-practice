use std::collections::HashMap;
use std::hash::Hash;

pub struct Graph <Vid, E=(), V=()> {
    verticies: HashMap<Vid, V>,
    adjacency: HashMap<Vid, Vec<(Vid, E)>>,
}

impl<Vid, E, V> Graph<Vid, E, V>
    where  
        Vid: Eq + Hash,
        V: Hash,
    {
        pub fn new() -> Graph<Vid, E, V> {
            Graph { verticies: HashMap::new(), adjacency: HashMap::new() }
        }

        pub fn push_vertex(self: &mut Graph<Vid, E, V>, vid: Vid, vertex: V){
            self.verticies.insert(vid, vertex);
        }

        pub fn push_edge(self: &mut Self, from: Vid, to: Vid, edge: E){
            let adjacent_to_from = self.adjacency.entry(from).or_default();
            adjacent_to_from.push((to, edge));
        }
    }

impl<Vid, E> Graph<Vid, E, ()>
    where
        Vid: Eq + Hash,
    {
        pub fn push_vid(self: &mut Self, vid: Vid){
            self.verticies.insert(vid, ());
        }
    }
fn main() {
    println!("Hello, world!");
}
