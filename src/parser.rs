#![feature(slice_patterns)]

use lexer::{Lexer, Token};
use node::{BinOp, Node, NodeBase};

pub struct Parser {
    pos: usize,
}

impl Parser {
    pub fn new() -> Self {
        Parser { pos: 0 }
    }

    pub fn run(&mut self, tokens: Vec<Token>) -> Result<Node, ()> {
        self.statements(&tokens)
    }
}

impl Parser {
    fn statements(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        let mut stmts: Vec<Box<Node>> = vec![];
        while !self.is_eof(&tokens) {
            let stmt = self.statement(&tokens)?;
            stmts.push(Box::new(stmt));
        }

        Ok(Node::new(NodeBase::Statements(stmts)))
    }

    fn statement(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        let stmt = match &tokens[self.pos] {
            Token::Return => {
                self.step();
                Node::new(NodeBase::Return(Box::new(self.expr(&tokens)?)))
            }
            _ => self.expr(&tokens)?,
        };
        self.expect(&tokens, Token::SemiColon);
        Ok(stmt)
    }

    fn expr(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        self.expr_op1(&tokens)
    }

    fn expr_op2(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        let mut lhs = self.number(&tokens)?;
        while !self.is_eof(&tokens) {
            match &tokens[self.pos] {
                Token::Asterisk => {
                    self.step();
                    lhs = Node::new(NodeBase::BinaryOp(
                        BinOp::Mul,
                        Box::new(lhs),
                        Box::new(self.number(&tokens)?),
                    ));
                }
                Token::Slash => {
                    self.step();
                    lhs = Node::new(NodeBase::BinaryOp(
                        BinOp::Div,
                        Box::new(lhs),
                        Box::new(self.number(&tokens)?),
                    ));
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn expr_op1(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        let mut lhs = self.expr_op2(&tokens)?;
        while !self.is_eof(&tokens) {
            match &tokens[self.pos] {
                Token::Plus => {
                    self.step();
                    lhs = Node::new(NodeBase::BinaryOp(
                        BinOp::Add,
                        Box::new(lhs),
                        Box::new(self.expr_op2(&tokens)?),
                    ));
                }
                Token::Minus => {
                    self.step();
                    lhs = Node::new(NodeBase::BinaryOp(
                        BinOp::Sub,
                        Box::new(lhs),
                        Box::new(self.expr_op2(&tokens)?),
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
    fn number(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        match &tokens[self.pos] {
            Token::Num(n) => {
                self.step();
                Ok(Node::new(NodeBase::Number(*n)))
            }
            _ => Err(()),
        }
    }
}

impl Parser {
    fn step(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, tokens: &Vec<Token>, token: Token) {
        if tokens[self.pos] != token {
            panic!("{:?} expected, but got {:?}", token, tokens[self.pos]);
        }
        self.step();
    }

    fn is_eof(&self, tokens: &Vec<Token>) -> bool {
        tokens[self.pos] == Token::EOF
    }
}
