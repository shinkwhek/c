// Register allocator

use std::collections::HashMap;

use gen_ir::{Ir, Op};

const reg_map_size: usize = 8192;
const num_regs: isize = 7;

pub struct RegAlloc {
    map: HashMap<isize, isize>,
    used: Vec<isize>,
}

impl RegAlloc {
    pub fn new() -> Self {
        RegAlloc {
            map: HashMap::new(),
            used: vec![],
        }
    }

    pub fn run(&mut self, irvv: Vec<Vec<Ir>>) -> Result<Vec<Vec<Ir>>, ()> {
        let mut vv = vec![];
        let mut v = vec![];
        for irv in irvv {
            for ir in irv {
                if let Ok(i) = self.reg_alloc(ir) {
                    v.push(i);
                }
            }
            vv.push(v);
            v = vec![];
        }
        Ok(vv)
    }
}

impl RegAlloc {
    fn reg_alloc(&mut self, mut ir: Ir) -> Result<Ir, ()> {
        match ir.op {
            Op::Imm | Op::Return | Op::Call(_) => {
                ir.lhs = self.alloc(ir.lhs)?;
                Ok(ir)
            }
            Op::Mov | Op::Add | Op::Sub | Op::Mul | Op::Div => {
                ir.lhs = self.alloc(ir.lhs)?;
                ir.rhs = self.alloc(ir.rhs)?;
                Ok(ir)
            }
            Op::Kill => {
                self.kill(ir.lhs);
                ir.op = Op::Nop;
                Ok(ir)
            }
            _ => Ok(ir),
        }
    }

    fn alloc(&mut self, ir_reg: isize) -> Result<isize, ()> {
        if reg_map_size <= ir_reg as usize {
            return Err(());
        }

        if let Some(ir) = self.map.get(&ir_reg) {
            return Ok(*ir);
        }

        for i in 0..num_regs {
            if self.used.iter().any(|&c| c == i) {
                continue;
            }
            self.used.push(i);
            self.map.insert(ir_reg, i);
            return Ok(i);
        }
        return Err(());
    }

    fn kill(&mut self, ir_reg: isize) {
        self.used.retain(|&i| i != ir_reg);
    }
}
