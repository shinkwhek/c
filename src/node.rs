#[derive(Debug, PartialEq)]
pub enum NodeBase {
    Number(usize),
    BinaryOp(BinOp, Box<Node>, Box<Node>),
    Return(Box<Node>),
    Statements(Vec<Box<Node>>),
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
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
