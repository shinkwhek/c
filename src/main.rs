use std::env;

extern crate c;
use c::lexer;
use c::node;
use c::parser;

fn main() {
    let code: String = env::args().skip(1).collect();

    if let Ok(lex) = lexer::Lexer::new(&code).run() {
        println!("lexer:\n{:?}", lex);
        if let Ok(parse) = parser::Parser::new(lex).expr_op2() {
            println!("parser:\n{:?}", parse);

            //            println!(".intel_syntax noprefix");
            //            println!(".global main");
            //            println!("main:");
            //
            //            println!("  mov rax, {}", args[1]);
            //            println!("  ret");
        }
    }
}
