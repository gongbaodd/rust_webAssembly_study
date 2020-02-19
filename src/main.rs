mod linked_node;

use crate::linked_node::{LNode, LNodeType};

fn main() {
    let mut node = LNode::new(0);
    let mut v: Vec<u32> = (1..5).collect();

    while let Some(i) = v.pop() {
        node.next = LNodeType::Next(Box::new(LNode::new(i)));
    }

    match node.next {
        LNodeType::Next(next_node) => println!("{}", next_node.value),
        LNodeType::Null => {
            println!("END");
        }
    }
}
