// generate x86 assembly from IR

use gen_ir::{Ir, Op};
use std::fmt;

struct Reg {
    name: String,
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

macro_rules! new_reg {
    ($s:expr) => {
        Reg {
            name: $s.to_string(),
        }
    };
}

pub struct X86 {
    regs: Vec<Reg>,
}

impl X86 {
    pub fn new() -> Self {
        X86 {
            regs: vec![
                new_reg!("rdi"),
                new_reg!("rsi"),
                new_reg!("r10"),
                new_reg!("r11"),
                new_reg!("r12"),
                new_reg!("r13"),
                new_reg!("r14"),
                new_reg!("r15"),
            ],
        }
    }
}

impl X86 {
    pub fn gen(&mut self, irv: Vec<Ir>) {
        for ir in &irv {
            match ir.op {
                Op::Imm => {
                    println!("  mov {}, {}", self.reg(ir.lhs), ir.rhs);
                }
                Op::Mov => {
                    println!("  mov {}, {}", self.reg(ir.lhs), self.reg(ir.rhs));
                }
                Op::Add => {
                    println!("  add {}, {}", self.reg(ir.lhs), self.reg(ir.rhs));
                }
                Op::Sub => {
                    println!("  sub {}, {}", self.reg(ir.lhs), self.reg(ir.rhs));
                }
                Op::Return => {
                    println!("  mov rax, {}", self.reg(ir.lhs));
                    println!("  ret");
                }
                Op::Nop => continue,
                _ => panic!("unknown operator"),
            }
        }
    }
}

impl X86 {
    fn reg(&self, ir_reg: usize) -> String {
        let s = &self.regs[ir_reg].name;
        s.to_string()
    }
}