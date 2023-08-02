mod lexer;
mod parser;
mod ast;
mod alpha;
mod beta;
mod pretty;

use lexer::Lexer;
use parser::Parser;
use alpha::AlphaConverter;
use beta::{BetaReducer, NormalExpressionNode};
use pretty::{
    pretty_print_normal,
    pretty_print
};

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};

fn main() -> rustyline::Result<()>{
    //let expression = r"(\x. (\z. z x) x) ((\y. y) 1 + 1)";
    //let expression = r"(\z. (\y. (\x. x y) y z)) 1";
    // let expression = r"(\x. \y. \z. x y z) 1 2 3";
    // let expression = "x y z";
    // let expression = r"(\f. \x. f (f x)) (\y. y * 2) 3";
    // let expression = r"(\x. x + 3) ((\y. y * 2) 5) + (\z. z / 2) (4 - 1)";
    // let expression = r"let double = \x. x * 2 in double 5";

    // let expression = r"let add = \x. \y. x + y in
    //                         let sub = \x. \y. x - y in
    //                         let mul = \x. \y. x * y in
    //                         let square = \x. mul x x in
    //                         let cube = \x. mul (mul x x) x in
    //                         let x = 5 in
    //                         let y = 3 in
    //                         let z = add (square x) (cube y) in
    //                         z";    

    // let _ = eval(&expression.to_string());

    println!("Lambdastampa: a λ-expr REPL.");
    println!("Type :exit or :x to exit.");

    // Initialise rustyline
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    // REPL loop
    loop {
        let readline = rl.readline("λ-expr >> ");
        match readline {
            Ok(line) => {                
                // check for builtin commands (only :exit for now)
                match line {
                    ref s if s == ":exit" || s == ":quit" || s == ":q" || s == ":x" => break,
                    _ => { }
                }

                // add input to command history
                rl.add_history_entry(line.as_str());

                // evaluate lambda expression
                let _ = eval(&line.to_string());
            },
            Err(ReadlineError::Interrupted) => {
                println!("Terminating...");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Terminating...");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");

    Ok(())
}

///
///  Evaluate a lambda expression
/// 
fn eval(lambda_expression: &String) -> Result<NormalExpressionNode, ()> {
    // Generate token list from input string
    let token_list = Lexer::new(lambda_expression.to_string())
        .scan()
        .unwrap();

    // Parse token list into abstract syntax tree
    let ast = Parser::new(token_list)
        .parse()
        .unwrap();

    // Pretty print the parsed input
    println!("λ-expr :");
    print!(">>> "); pretty_print(&ast);
    println!();

    // Perform alpha conversion on the abstract syntax tree
    let _ = AlphaConverter::new()
        .convert(&ast);

    // Print the normal form
    println!("α-conversion :");
    print!(">>> "); pretty_print(&ast);
    println!();

    // Perform beta reduction on the abstract syntax tree
    let mut beta_reducer = BetaReducer::new();
    let normal_form = beta_reducer.convert(&ast);
    let result = beta_reducer.reduce(&normal_form);

    // Print the normal form
    println!("Normal Form (after β-reductions) :");
    print!(">>> "); pretty_print_normal(&result);
    println!();

    Ok(result)
}

use clap::Parser as ClapParser;
