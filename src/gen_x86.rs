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
    regs8: Vec<Reg>,
    regs32: Vec<Reg>,
    argregs: Vec<Reg>,
    argregs8: Vec<Reg>,
    argregs32: Vec<Reg>,
    nlabel: usize,
}

impl X86 {
    pub fn new() -> Self {
        X86 {
            regs: vec![
                new_reg!("r10"),
                new_reg!("r11"),
                new_reg!("rbx"),
                new_reg!("r12"),
                new_reg!("r13"),
                new_reg!("r14"),
                new_reg!("r15"),
            ],
            regs8: vec![
                new_reg!("r10b"),
                new_reg!("r11b"),
                new_reg!("bl"),
                new_reg!("r12b"),
                new_reg!("r13b"),
                new_reg!("r14b"),
                new_reg!("r15b"),
            ],
            regs32: vec![
                new_reg!("r10d"),
                new_reg!("r11d"),
                new_reg!("ebx"),
                new_reg!("r12d"),
                new_reg!("r13d"),
                new_reg!("r14d"),
                new_reg!("r15d"),
            ],
            argregs: vec![
                new_reg!("rdi"),
                new_reg!("rsi"),
                new_reg!("rdx"),
                new_reg!("rcx"),
                new_reg!("r8"),
                new_reg!("r9"),
            ],
            argregs8: vec![
                new_reg!("dil"),
                new_reg!("sil"),
                new_reg!("dl"),
                new_reg!("r8b"),
                new_reg!("r9b"),
            ],
            argregs32: vec![
                new_reg!("edi"),
                new_reg!("esi"),
                new_reg!("edx"),
                new_reg!("ecx"),
                new_reg!("r8d"),
                new_reg!("r9d"),
            ],
            nlabel: 1,
        }
    }
}

impl X86 {
    pub fn emit(&mut self, irvv: &Vec<Vec<Ir>>) {
        self.nlabel += 1;

        println!(".intel_syntax noprefix");
        println!(".global main");

        for irv in irvv.iter() {
            self.emit_ir(&irv);
        }
    }

    fn emit_ir(&mut self, irv: &Vec<Ir>) {
        for ir in irv {
            match &ir.op {
                Op::DefFun(s) => {
                    println!("{}:", s);
                    println!("  push rbp");
                    println!("  mov rbp, rsp");
                }
                Op::Call(s, args) => {
                    for arg in args {
                        println!("  mov {}, {}", self.argreg(*arg, 4), self.reg(*arg, 4));
                    }

                    println!("  mov rax, 0");
                    println!("  call {}", s);
                    println!("  mov {}, eax", self.reg(ir.lhs, 4));
                }
                Op::Imm => {
                    println!("  mov {}, {}", self.reg(ir.lhs, 4), ir.rhs);
                }
                Op::StoreArg => {
                    println!(
                        "  mov dword ptr [rbp-{}], {}",
                        (ir.lhs + 1) * 4,
                        self.argreg(ir.lhs, 4)
                    );
                }
                Op::Load => {
                    println!(
                        "  mov {}, dword ptr [rbp-{}]",
                        self.reg(ir.lhs, 4),
                        (ir.lhs + 1) * 4
                    );
                }
                Op::Mov => {
                    println!("  mov {}, {}", self.reg(ir.lhs, 4), self.reg(ir.rhs, 4));
                }
                Op::Return => {
                    println!("  mov eax, {}", self.reg(ir.lhs, 4));
                    //println!("  jmp {}");
                }
                Op::Add => {
                    println!("  add {}, {}", self.reg(ir.lhs, 4), self.reg(ir.rhs, 4));
                }
                Op::Sub => {
                    println!("  sub {}, {}", self.reg(ir.lhs, 4), self.reg(ir.rhs, 4));
                }
                Op::Mul => {
                    println!("  mov eax, {}", self.reg(ir.rhs, 4));
                    println!("  imul {}", self.reg(ir.lhs, 4));
                    println!("  mov {}, eax", self.reg(ir.lhs, 4));
                }
                Op::Div => {
                    println!("  mov eax, {}", self.reg(ir.lhs, 4));
                    println!("  cqo");
                    println!("  div {}", self.reg(ir.rhs, 4));
                    println!("  mov {}, eax", self.reg(ir.lhs, 4));
                }
                Op::Nop => continue,
                _ => panic!("unknown operator"),
            }
        }
        println!("  pop rbp");
        println!("  ret");
    }
}

impl X86 {
    fn reg(&self, ir_reg: isize, size: usize) -> String {
        let r = match size {
            1 => &self.regs8,
            4 => &self.regs32,
            _ => &self.regs,
        };
        let s = &r[ir_reg as usize].name;
        s.to_string()
    }
    fn argreg(&self, ir_reg: isize, size: usize) -> String {
        let r = match size {
            1 => &self.argregs8,
            4 => &self.argregs32,
            _ => &self.argregs,
        };
        let s = &r[ir_reg as usize].name;
        s.to_string()
    }
}
