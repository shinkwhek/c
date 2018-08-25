// Register allocator

use std::collections::HashMap;

use gen_ir::{Ir, Op};

pub struct RegAlloc {
    map: HashMap<usize, usize>,
    used: Vec<usize>,
}

impl RegAlloc {
    pub fn new() -> Self {
        RegAlloc {
            map: HashMap::new(),
            used: vec![],
        }
    }

    pub fn run(&mut self, irv: Vec<Ir>) -> Result<Vec<Ir>, ()> {
        let mut v = vec![];
        for ir in irv {
            v.push(self.reg_alloc(ir)?);
        }
        Ok(v)
    }
}

impl RegAlloc {
    fn reg_alloc(&mut self, mut ir: Ir) -> Result<Ir, ()> {
        match ir.op {
            Op::Imm => {
                ir.lhs = self.alloc(ir.lhs)?;
                Ok(ir)
            }
            Op::Add | Op::Sub | Op::Mul => {
                ir.lhs = self.alloc(ir.lhs)?;
                ir.rhs = self.alloc(ir.rhs)?;
                Ok(ir)
            }
            Op::Return => {
                self.kill(ir.lhs);
                Ok(ir)
            }
            Op::Kill => {
                self.kill(ir.lhs);
                ir.op = Op::Nop;
                Ok(ir)
            }
            _ => Err(()),
        }
    }

    fn alloc(&mut self, ir_reg: usize) -> Result<usize, ()> {
        if let Some(r) = self.map.get(&ir_reg) {
            return Ok(*r);
        }

        let mut i = 0;
        loop {
            if self.used.iter().all(|&c| c != i) {
                self.used.push(i);
                self.map.insert(ir_reg, i);
                return Ok(i);
            }
            i += 1;
        }
        return Err(());
    }

    fn kill(&mut self, ir_reg: usize) {
        self.used.retain(|&i| i != ir_reg);
    }
}
