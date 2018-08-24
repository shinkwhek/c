#[derive(Debug, PartialEq)]
pub enum NodeBase {
    BinaryOp(Box<Node>, Box<Node>, BinOp),
    Number(usize),
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub base: NodeBase,
}

impl Node {
    pub fn new(base: NodeBase) -> Node {
        Node { base: base }
    }
}
