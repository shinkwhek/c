use std::env;

extern crate c;
use c::gen_ir;
use c::gen_x86;
use c::lexer;
use c::node;
use c::parser;
use c::regalloc;

fn main() {
    let code: String = env::args().skip(1).collect();

    if let Ok(lex) = lexer::Lexer::new(&code).run() {
        //println!("lexer:\n{:?}", lex);

        if let Ok(parse) = parser::Parser::new().run(lex) {
            //println!("parser:\n{:?}", parse);

            if let Ok(irv) = gen_ir::GenIr::new().run(parse) {
                //println!{"ir:"}
                //for ir in &irv {
                //    println!("{:?}", ir);
                //}

                if let Ok(irv) = regalloc::RegAlloc::new().run(irv) {
                    //println!("regAlloc:");
                    //for ir in &irv {
                    //    println!("{:?}", ir);
                    //}

                    gen_x86::X86::new().gen(irv);
                }
            }
        }
    }
}
