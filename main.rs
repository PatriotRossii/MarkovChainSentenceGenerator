use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct MarkovChainNode<T: std::cmp::PartialEq + Eq + Hash> {
    content: T,
    count_of: HashMap<T, i32>,
    next: Vec<(f64, MarkovChainNode<T>)>
}

impl<T: Clone + PartialEq + Eq + Hash> MarkovChainNode<T> {
    fn new(content: T) -> Self {
        Self {
            content,
            count_of: HashMap::new(),
            next: Vec::new(),
        }
    }
    fn add(&mut self, node: MarkovChainNode<T>) {
        let entry = self.count_of.entry(node.content.clone()).or_insert(0);
        *entry += 1;
        let length = self.next.len();
        
        self.next.push((f64::NAN, node));

        for element in &mut self.next {
            element.0 = *self.count_of.get(&element.1.content).unwrap() as f64 / length as f64
        }
    }
}

fn main() {
    let mut chain = MarkovChainNode::new("Hello");
    
    chain.add(MarkovChainNode::new("Hi"));
    chain.add(MarkovChainNode::new("Hi"));
    
    println!("{:?}", chain);
}
