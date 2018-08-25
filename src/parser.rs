use lexer::{Lexer, Token};
use node::{BinOp, Node, NodeBase};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            pos: 0,
        }
    }
}

impl Parser {
    pub fn expr_op2(&mut self) -> Result<Node, ()> {
        let mut lhs = self.number()?;
        while !self.is_eof() {
            match self.tokens[self.pos] {
                Token::Asterisk => {
                    self.step();
                    lhs = Node::new(NodeBase::BinaryOp(
                        BinOp::Mul,
                        Box::new(lhs),
                        Box::new(self.number()?),
                    ));
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    pub fn expr_op1(&mut self) -> Result<Node, ()> {
        let mut lhs = self.expr_op2()?;
        while !self.is_eof() {
            match self.tokens[self.pos] {
                Token::Plus => {
                    self.step();
                    lhs = Node::new(NodeBase::BinaryOp(
                        BinOp::Add,
                        Box::new(lhs),
                        Box::new(self.expr_op2()?),
                    ));
                }
                Token::Minus => {
                    self.step();
                    lhs = Node::new(NodeBase::BinaryOp(
                        BinOp::Sub,
                        Box::new(lhs),
                        Box::new(self.expr_op2()?),
                    ));
                }
                _ => {
                    break;
                }
            }
        }
        Ok(lhs)
    }
}

impl Parser {
    fn number(&mut self) -> Result<Node, ()> {
        match self.tokens[self.pos] {
            Token::Num(n) => {
                self.step();
                Ok(Node::new(NodeBase::Number(n)))
            }
            _ => Err(()),
        }
    }
}

impl Parser {
    fn step(&mut self) {
        self.pos += 1;
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
