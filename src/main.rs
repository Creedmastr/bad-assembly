#![allow(dead_code)]

use std::ops::Not;

use registers::{operations::RegisterOperationsInteger, reg_struct::Register};

use crate::registers::operations::RegisterOperationsString;

mod registers;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("ERROR: Wrong usage");
    } else if args[1].ends_with(".asm").not() {
        panic!("ERROR: Wrong file")
    }

    let mut int_register: Vec<Register<i32>> = vec![];
    let mut string_register: Vec<Register<String>> = vec![];

    let file_content = std::fs::read_to_string(&args[1]).unwrap();

    for line in file_content.lines() {
        let line = line.to_string();

        let line = line.to_lowercase();

        match line {
            x if x.starts_with("add ") || x.starts_with("sub ") => {
                let is_add = match x.starts_with("add ") {
                    true => true,
                    false => false,
                };

                let register_name = x.replace("add ", "").replace("sub ", "");

                let current_register = int_register.search(&register_name);

                let value = match is_add {
                    true => current_register.0.value + 1,
                    false => current_register.0.value - 1,
                };

                int_register.update(&Register {
                    name: register_name,
                    value,
                })
            }

            x if x.starts_with("mov ") => {
                let is_string = x.contains(r#"""#);

                let tokens: Vec<String> = x
                    .replace("mov ", "")
                    .replace(" ", "")
                    .replace(r#"""#, "")
                    .split(",")
                    .map(|f| f.to_string())
                    .collect();

                match is_string {
                    true => {
                        string_register.update(&Register {
                            name: tokens[0].clone(),
                            value: tokens[1].clone(),
                        });
                    }

                    false => {
                        int_register.update(&Register {
                            name: tokens[0].clone(),
                            value: tokens[1].parse().unwrap(),
                        });
                    }
                }
            }

            x if x.starts_with("mul ") || x.starts_with("div ") => {
                let is_mul = x.contains("mul");

                let mul_value = int_register
                    .search(&x.replace("mul ", "").replace("div ", ""))
                    .0
                    .value;
                let ax_value = int_register.search(&String::from("ax")).0.value;

                match is_mul {
                    true => {
                        int_register.update(&Register {
                            name: "ax".to_string(),
                            value: mul_value * ax_value,
                        });
                    }

                    false => {
                        int_register.update(&Register {
                            name: "ax".to_string(),
                            value: ax_value / mul_value,
                        });

                        int_register.update(&Register {
                            name: "ah".to_string(),
                            value: ax_value % mul_value,
                        });
                    }
                }
            }

            x if x.starts_with("imul ") || x.starts_with("idiv ") => {
                let is_mul = x.contains("imul ");

                let tokens: Vec<String> = x
                    .replace("imul ", "")
                    .replace("idiv ", "")
                    .replace(" ", "")
                    .split(",")
                    .map(|f| f.to_string())
                    .collect();

                let first_value = int_register.search(&tokens[0]).0.value;
                let second_value = int_register.search(&tokens[1]).0.value;

                match is_mul {
                    true => int_register.update(&Register {
                        name: tokens[0].clone(),
                        value: first_value * second_value,
                    }),

                    false => {
                        int_register.update(&Register {
                            name: tokens[0].clone(),
                            value: first_value / second_value,
                        });
                        int_register.update(&Register {
                            name: "ah".to_string(),
                            value: first_value % second_value,
                        });
                    }
                }
            }

            x if x.starts_with("_printf ") => {
                // Basically print a string if it exits, or else it prints a number (either it exists or 0)
                let register = x.replace("_printf ", "");

                let int_search = int_register.search(&register);
                let string_search = string_register.search(&register);

                if !string_search.1 {
                    println!("{}", string_search.0.value);
                    continue;
                }

                println!("{}", int_search.0.value);
            }

            _ => {
                continue;
            }
        }
    }

    if args.contains(&String::from("-debug")) {
        eprintln!(
            "\nArithmetic: {:#?}\nString: {:#?}",
            int_register, string_register
        );
    }
}
