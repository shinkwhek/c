use node::{BinOp, Node, NodeBase};

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Imm,
    Mov,
    Return,
    Kill,
    Nop,
}

#[derive(Debug, PartialEq)]
pub struct Ir {
    pub op: Op,
    pub lhs: usize,
    pub rhs: usize,
}

impl Ir {
    fn new(op: Op, lhs: usize, rhs: usize) -> Ir {
        Ir {
            op: op,
            lhs: lhs,
            rhs: rhs,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GenIr {
    regc: usize,
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
        let r = self.gen_sub(node)?;
        self.ins.push(Ir::new(Op::Return, r, 0));
        Ok(self.ins)
    }
}

impl GenIr {
    fn gen_sub(&mut self, node: Node) -> Result<usize, ()> {
        match node.base {
            NodeBase::Number(n) => {
                let current = self.regc;
                self.regc += 1;
                self.ins.push(Ir::new(Op::Imm, current, n));
                return Ok(current);
            }
            NodeBase::BinaryOp(op, lhs, rhs) => {
                let lhs: usize = self.gen_sub(*lhs)?;
                let rhs: usize = self.gen_sub(*rhs)?;

                let op = match op {
                    BinOp::Add => Op::Add,
                    BinOp::Sub => Op::Sub,
                    BinOp::Mul => Op::Mul,
                };

                self.ins.push(Ir::new(op, lhs, rhs));
                self.ins.push(Ir::new(Op::Kill, rhs, 0));
                return Ok(lhs);
            }
        }
    }
}
