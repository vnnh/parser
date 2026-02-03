use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::collections::HashMap;
use std::f64::consts::{E, PI};

mod ast;
use ast::{Expr, Opcode};

// LALRPOP generates a struct named StatementParser
lalrpop_util::lalrpop_mod!(pub grammar);

fn main() {
    // CLI header
    println!("Parser (Vincent Dinh)");
    println!("Enhancements: Greek symbols (π, Δ), Factorials (!), Constants (pi, e)");
    println!("Type 'exit' to stop.\n");

    // initialize symbol table with predefined mathematical constants
    let mut vars: HashMap<String, f64> = HashMap::new();
    vars.insert("pi".to_string(), PI);
    vars.insert("π".to_string(), PI);
    vars.insert("e".to_string(), E);

    // setup Read-Eval-Print Loop (REPL)
    let mut rl = DefaultEditor::new().unwrap();
    let parser = grammar::StatementParser::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                if input == "exit" {
                    break;
                }

                rl.add_history_entry(input).unwrap();

                // use parser to construct parse tree
                match parser.parse(input) {
                    Ok(ast) => {
                        // println!("DEBUG AST: {:?}", ast);

                        // evaluate expression with parse tree, and pass symbol table for assignments
                        match evaluate(&ast, &mut vars) {
                            Ok(val) => println!("Result: {}", val),
                            Err(e) => println!("Runtime Error: {}", e),
                        }
                    }
                    Err(e) => println!("Syntax Error: {}", e),
                }
            }
            Err(ReadlineError::Interrupted) => break, // Ctrl-C
            Err(ReadlineError::Eof) => break,         // Ctrl-D
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn evaluate(expr: &Expr, vars: &mut HashMap<String, f64>) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Var(name) => vars
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undeclared variable: '{}'", name)),
        Expr::Assign(name, val_expr) => {
            let val = evaluate(val_expr, vars)?;
            vars.insert(name.clone(), val);
            Ok(val)
        }
        Expr::Op(lhs, op, rhs) => {
            let l = evaluate(lhs, vars)?;
            let r = evaluate(rhs, vars)?;
            match op {
                Opcode::Add => Ok(l + r),
                Opcode::Sub => Ok(l - r),
                Opcode::Mul => Ok(l * r),
                Opcode::Div => {
                    if r == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(l / r)
                    }
                }
                Opcode::Mod => Ok(l % r),
                Opcode::Pow => Ok(l.powf(r)),
            }
        }
        Expr::Neg(expr) => {
            let val = evaluate(expr, vars)?;
            Ok(-val)
        }
        Expr::Factorial(expr) => {
            let val = evaluate(expr, vars)?;
            if val < 0.0 || val.fract() != 0.0 {
                return Err("Factorial requires a non-negative integer".to_string());
            }
            Ok(factorial(val as u64) as f64)
        }
        Expr::Call(func, arg_expr) => {
            let val = evaluate(arg_expr, vars)?;
            match func.as_str() {
                "sin" => Ok(val.sin()),
                "cos" => Ok(val.cos()),
                "sqrt" => {
                    if val < 0.0 {
                        Err("Sqrt of negative number".to_string())
                    } else {
                        Ok(val.sqrt())
                    }
                }
                "ln" => {
                    if val <= 0.0 {
                        Err("Log of non-positive number".to_string())
                    } else {
                        Ok(val.ln())
                    }
                }
                _ => Err(format!("Unknown function: {}", func)),
            }
        }
    }
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}
