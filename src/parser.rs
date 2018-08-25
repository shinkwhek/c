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
    pub fn statements(&mut self) -> Result<Node, ()> {
        let mut stmts: Vec<Box<Node>> = vec![];
        while !self.is_eof() {
            let stmt = self.statement()?;
            stmts.push(Box::new(stmt));
        }

        Ok(Node::new(NodeBase::Statements(stmts)))
    }

    fn statement(&mut self) -> Result<Node, ()> {
        let stmt = match self.tokens[self.pos] {
            Token::Return => {
                self.step();
                Node::new(NodeBase::Return(Box::new(self.expr()?)))
            }
            _ => self.expr()?,
        };
        self.expect(Token::SemiColon);
        Ok(stmt)
    }

    fn expr(&mut self) -> Result<Node, ()> {
        self.expr_op1()
    }

    fn expr_op2(&mut self) -> Result<Node, ()> {
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
                Token::Slash => {
                    self.step();
                    lhs = Node::new(NodeBase::BinaryOp(
                        BinOp::Div,
                        Box::new(lhs),
                        Box::new(self.number()?),
                    ));
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn expr_op1(&mut self) -> Result<Node, ()> {
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

    fn expect(&mut self, token: Token) {
        if self.tokens[self.pos] != token {
            panic!("{:?} expected, but got {:?}", token, self.tokens[self.pos]);
        }
        self.step();
    }

    fn is_eof(&self) -> bool {
        self.tokens[self.pos] == Token::EOF
    }
}
