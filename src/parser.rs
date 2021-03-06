#![feature(slice_patterns)]

use lexer::{Lexer, Token};
use node::{BinOp, Ctype, Node, NodeBase};

pub struct Parser {
    pos: usize,
}

impl Parser {
    pub fn new() -> Self {
        Parser { pos: 0 }
    }

    pub fn run(&mut self, tokens: Vec<Token>) -> Result<Vec<Node>, ()> {
        let mut v = vec![];
        while !self.is_eof(&tokens) {
            let gd = self.global_def(&tokens)?;
            v.push(gd);
        }
        Ok(v)
    }
}

impl Parser {
    fn global_def(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        match &tokens[self.pos] {
            Token::Ctype(s) => {
                self.step();
                let typ = Node::ctype(&s)?;
                let id = self.ident(&tokens)?;
                self.expect(&tokens, Token::LeftParen);
                let local_args = self.args_def(&tokens)?;
                self.expect(&tokens, Token::RightParen);
                self.expect(&tokens, Token::LeftCurlyBrace);
                let stmts = self.statements(&tokens, Token::RightCurlyBrace)?;
                self.expect(&tokens, Token::RightCurlyBrace);
                Ok(Node::new(NodeBase::DefFun(
                    typ,
                    Box::new(id),
                    local_args,
                    Box::new(stmts),
                )))
            }
            _ => Err(()),
        }
    }

    fn args_def(&mut self, tokens: &Vec<Token>) -> Result<Vec<(Ctype, Node)>, ()> {
        let mut v = vec![];
        while !self.consume(&tokens, Token::RightParen, 0) {
            let argtyp = self.ctype(&tokens)?;
            let argid = self.ident(&tokens)?;
            v.push((argtyp, argid));
            if self.consume(&tokens, Token::Comma, 0) {
                self.expect(&tokens, Token::Comma);
            }
        }
        Ok(v)
    }

    fn statements(&mut self, tokens: &Vec<Token>, end: Token) -> Result<Node, ()> {
        let mut stmts: Vec<Box<Node>> = vec![];
        while end != tokens[self.pos] {
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

    fn expr_op2(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        let mut lhs = self.term(&tokens)?;
        while !self.is_eof(&tokens) {
            match &tokens[self.pos] {
                Token::Asterisk => {
                    self.step();
                    lhs = Node::new(NodeBase::BinaryOp(
                        BinOp::Mul,
                        Box::new(lhs),
                        Box::new(self.term(&tokens)?),
                    ));
                }
                Token::Slash => {
                    self.step();
                    lhs = Node::new(NodeBase::BinaryOp(
                        BinOp::Div,
                        Box::new(lhs),
                        Box::new(self.term(&tokens)?),
                    ));
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
}

impl Parser {
    fn term(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        match &tokens[self.pos] {
            Token::Num(_) => self.number(&tokens),
            Token::Ident(_) => {
                if self.consume(&tokens, Token::LeftParen, 1) {
                    return self.funccall(&tokens);
                }
                self.ident(&tokens)
            }
            _ => Err(()),
        }
    }

    fn number(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        match &tokens[self.pos] {
            Token::Num(n) => {
                self.step();
                Ok(Node::new(NodeBase::Number(*n)))
            }
            _ => Err(()),
        }
    }

    fn ident(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        match &tokens[self.pos] {
            Token::Ident(s) => {
                self.step();
                Ok(Node::new(NodeBase::Ident(s.to_string())))
            }
            _ => Err(()),
        }
    }

    fn funccall(&mut self, tokens: &Vec<Token>) -> Result<Node, ()> {
        match &tokens[self.pos] {
            Token::Ident(s) => {
                self.step();
                self.expect(&tokens, Token::LeftParen);
                let call_arg = self.call_arg(&tokens)?;
                self.expect(&tokens, Token::RightParen);
                Ok(Node::new(NodeBase::Call(s.to_string(), call_arg)))
            }
            _ => Err(()),
        }
    }

    fn call_arg(&mut self, tokens: &Vec<Token>) -> Result<Vec<Node>, ()> {
        let mut v = vec![];
        while !self.consume(&tokens, Token::RightParen, 0) {
            let exp = self.expr(&tokens)?;
            v.push(exp);
            if self.consume(&tokens, Token::Comma, 0) {
                self.expect(&tokens, Token::Comma);
            } else {
                break;
            }
        }
        Ok(v)
    }

    fn ctype(&mut self, tokens: &Vec<Token>) -> Result<Ctype, ()> {
        match &tokens[self.pos] {
            Token::Ctype(s) => {
                self.step();
                Node::ctype(&s)
            }
            _ => Err(()),
        }
    }
}

impl Parser {
    fn step(&mut self) {
        self.pos += 1;
    }

    fn consume(&self, tokens: &Vec<Token>, token: Token, n: usize) -> bool {
        if tokens[self.pos + n] == token {
            return true;
        }
        false
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
