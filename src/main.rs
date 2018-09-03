use std::env;

extern crate c;
use c::gen_ir;
use c::gen_x86;
use c::lexer;
use c::node;
use c::parser;
use c::regalloc;

extern crate clap;
use clap::{App, Arg};

use std::fs::OpenOptions;
use std::io::prelude::*;

const VERSION_STR: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let app = App::new("c")
        .version(VERSION_STR)
        .author("shinkwhek")
        .about("A toy C compiler")
        .arg(Arg::with_name("file").help("input file name").index(1));
    let app_matches = app.clone().get_matches();

    if let Some(filename) = app_matches.value_of("file") {
        let mut code = String::new();
        match OpenOptions::new().read(true).open(filename) {
            Ok(mut ok) => {
                ok.read_to_string(&mut code).ok().expect("cannot read file");
            }
            Err(e) => {
                println!("error: {}", e);
                ::std::process::exit(0);
            }
        }

        if let Ok(lex) = lexer::Lexer::new(&code).run() {
            //println!("lexer:\n{:?}", lex);

            if let Ok(parse) = parser::Parser::new().run(lex) {
                //println!("parser:\n{:?}", parse);

                if let Ok(irv) = gen_ir::GenIr::new().run(&parse) {
                    //println!{"ir:"}
                    //for ir in &irv {
                    //    for i in ir {
                    //        println!("{:?}", i);
                    //    }
                    //}

                    if let Ok(irv) = regalloc::RegAlloc::new().run(irv) {
                        //println!("regAlloc:");
                        //for ir in &irv {
                        //    for i in ir {
                        //        println!("{:?}", i);
                        //    }
                        //}

                        gen_x86::X86::new().emit(&irv);
                    }
                }
            }
        }
    }
}
