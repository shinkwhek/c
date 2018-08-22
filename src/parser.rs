#![feature(slice_patterns)]

use lexer::{Lexer, Token};

pub enum Node {
    NdNum(usize),
    NdAdd(Box<Node>, Box<Node>),
    NdSub(Box<Node>, Box<Node>),
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens: tokens }
    }
}

impl Parser {}
