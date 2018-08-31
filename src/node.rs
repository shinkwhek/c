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
pub enum Ctype {
    Int,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub base: NodeBase,
}

impl Node {
    pub fn new(base: NodeBase) -> Node {
        Node { base: base }
    }

    pub fn ctype(s: &str) -> Result<Ctype, ()> {
        match s {
            "int" => Ok(Ctype::Int),
            _ => Err(()),
        }
    }
}
