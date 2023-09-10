use std::ops::Not;

use registers::{operations::RegisterOperationsArithmetic, reg_struct::Register};

mod registers;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("ERROR: Wrong usage");
    } else if args[1].ends_with(".asm").not() {
        panic!("ERROR: Wrong file")
    }

    let mut arithmetics_register: Vec<Register<i32>> = vec![];

    let file_content = std::fs::read_to_string(&args[1]).unwrap();

    for line in file_content.lines() {
        let line = line.to_string();

        match line {
            x if x.starts_with("ADD ") => {
                let register_name = x.replace("ADD ", "");

                let current_register = arithmetics_register.search(&register_name);

                arithmetics_register.update(
                    register_name.clone(),
                    &Register {
                        name: register_name,
                        value: current_register.value + 1,
                    },
                )
            }

            x if x.starts_with("MOV ") => {
                let tokens: Vec<String> = x
                    .replace("MOV ", "")
                    .replace(" ", "")
                    .split(",")
                    .map(|f| f.to_string())
                    .collect();

                let current_register = arithmetics_register.search(&tokens[0]);
                
                arithmetics_register.update(
                    tokens[0].clone(),
                    &Register {
                        name: tokens[0].clone(),
                        value: tokens[1].parse().unwrap(),
                    },
                );
            }

            _ => {
                continue;
            }
        }
    }

    println!("{:#?}", arithmetics_register);
}
