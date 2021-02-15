use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct MarkovChainNode<T: std::cmp::PartialEq + Eq + Hash + Copy> {
    content: T,
    count_of: HashMap<T, i32>,
    next: Vec<(f64, MarkovChainNode<T>)>
}

impl<T: Copy + PartialEq + Eq + Hash> MarkovChainNode<T> {
    fn new(content: T) -> Self {
        Self {
            content,
            count_of: HashMap::new(),
            next: Vec::new(),
        }
    }
    fn add(&mut self, node: MarkovChainNode<T>) -> &mut MarkovChainNode<T> {
        let entry = self.count_of.entry(node.content).or_insert(0);
        *entry += 1;        
        self.next.push((f64::NAN, node));

        let length = self.next.len();

        for element in &mut self.next {
            element.0 = *self.count_of.get(&element.1.content).unwrap() as f64 / length as f64
        }

        &mut self.next.last_mut().unwrap().1
    }
    fn add_chain(&mut self, vec: Vec<MarkovChainNode<T>>) {
        if vec.is_empty() { return }

        let mut last = self;
        for node in vec {
            let new_node = last.add(node);
            last = new_node;
        }
    }
}

fn main() {
    let mut chain = MarkovChainNode::new("Привет");

    chain.add_chain(vec![MarkovChainNode::new("Друг"), MarkovChainNode::new("Как"), MarkovChainNode::new("Дела")]);
    chain.add_chain(vec![MarkovChainNode::new("Пока"), MarkovChainNode::new("Пидарас"), MarkovChainNode::new("Заебал")]);
    println!("{:?}", chain);
}
