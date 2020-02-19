pub enum LNodeType {
    Next(Box<LNode>),
    Null,
}

pub struct LNode {
    pub value: u32,
    pub next: LNodeType,
}

impl LNode {
    pub fn new(v: u32) -> LNode {
        LNode {
            value: v,
            next: LNodeType::Null,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_node() {
        let node = LNode::new(0);
        assert_eq!(node.value, 0);
        let next = node.next;
        match next {
            LNodeType::Next(_) => panic!("node should not be a LNodeType::Next"),
            LNodeType::Null => {}
        }
    }
}
