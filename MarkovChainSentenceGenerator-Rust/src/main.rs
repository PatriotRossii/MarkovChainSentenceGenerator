use std::{collections::HashMap, rc::Rc};
use std::hash::Hash;

#[derive(Debug, Clone)]
struct MarkovChainNextNode<T: PartialEq + Eq + Hash + Copy> {
    probability: f64,
    node: Rc<MarkovChainNode<T>>,
}

#[derive(Debug, Clone)]
pub struct MarkovChainNode<T: PartialEq + Eq + Hash + Copy> {
    content: T,
    count_of: HashMap<T, i32>,
    next: Vec<MarkovChainNextNode<T>>
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
        self.next.push(MarkovChainNextNode { probability: f64::NAN, node: Rc::new(node) });

        let length = self.next.len();

        for element in &mut self.next {
            element.probability = *self.count_of.get(&element.node.content).unwrap() as f64 / length as f64
        }

        Rc::get_mut(&mut self.next.last_mut().unwrap().node).unwrap()
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

    let mut gay = MarkovChainNode::new("Пидарас");
    gay.add_chain(vec![MarkovChainNode::new("Чтобы"), MarkovChainNode::new("Ты"), MarkovChainNode::new("Сдох")]);

    chain.add_chain(vec![MarkovChainNode::new("Пока"), gay, MarkovChainNode::new("Заебал")]);
    println!("{:?}", chain);
}
