#[derive(Debug, PartialEq)]
pub enum NodeBase {
    BinaryOp(BinOp, Box<Node>, Box<Node>),
    Number(usize),
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
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
