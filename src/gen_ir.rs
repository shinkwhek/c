use node::{BinOp, Node, NodeBase};

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Imm,
    Mov,
    Return,
    Kill,
    Nop,
}

#[derive(Debug, PartialEq)]
pub struct Ir {
    pub op: Op,
    pub lhs: isize,
    pub rhs: isize,
}

impl Ir {
    fn new(op: Op, lhs: isize, rhs: isize) -> Ir {
        Ir {
            op: op,
            lhs: lhs,
            rhs: rhs,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GenIr {
    regc: isize,
    ins: Vec<Ir>,
}

impl GenIr {
    pub fn new() -> Self {
        GenIr {
            regc: 0,
            ins: vec![],
        }
    }

    pub fn run(mut self, node: Node) -> Result<Vec<Ir>, ()> {
        //let r = self.expr(node)?;
        //self.ins.push(Ir::new(Op::Return, r, 0));
        self.statement(node)?;
        Ok(self.ins)
    }
}

impl GenIr {
    fn statement(&mut self, node: Node) -> Result<(), ()> {
        match node.base {
            NodeBase::Return(e) => {
                let r = self.expr(*e)?;
                self.ins.push(Ir::new(Op::Return, r, 0));
                self.ins.push(Ir::new(Op::Kill, r, -1));
                return Ok(());
            }
            NodeBase::Statements(v) => {
                for nd in v {
                    self.statement(*nd)?;
                }
                return Ok(());
            }
            _ => {
                let r = self.expr(node)?;
                self.ins.push(Ir::new(Op::Kill, r, -1));
                return Ok(());
            }
        }
    }

    fn expr(&mut self, node: Node) -> Result<isize, ()> {
        match node.base {
            NodeBase::Number(n) => {
                let current = self.regc;
                self.regc += 1;
                self.ins.push(Ir::new(Op::Imm, current, n as isize));
                return Ok(current);
            }
            NodeBase::BinaryOp(op, lhs, rhs) => {
                return self.binary_op(op, *lhs, *rhs);
            }
            _ => return Err(()),
        }
    }

    fn binary_op(&mut self, op: BinOp, lhs: Node, rhs: Node) -> Result<isize, ()> {
        let lhs: isize = self.expr(lhs)?;
        let rhs: isize = self.expr(rhs)?;
        let op = match op {
            BinOp::Add => Op::Add,
            BinOp::Sub => Op::Sub,
            BinOp::Mul => Op::Mul,
            BinOp::Div => Op::Div,
        };

        self.ins.push(Ir::new(op, lhs, rhs));
        self.ins.push(Ir::new(Op::Kill, rhs, -1));
        Ok(lhs)
    }
}
