///
/// Lambdastampa: a λ-expr REPL.
///
/// Some supported expressions: 
/// 
///     (\x. (\z. z x) x) ((\y. y) 1 + 1)
/// 
///     (\z. (\y. (\x. x y) y z)) 1
/// 
///     (\x. \y. \z. x y z) 1 2 3
/// 
///     x y z
/// 
///     (\f. \x. f (f x)) (\y. y * 2) 3
/// 
///     let double = \x. x * 2 in double 5
///
///     let add = \x. \y. x + y in
///         let sub = \x. \y. x - y in
///         let mul = \x. \y. x * y in
///         let square = \x. mul x x in
///         let cube = \x. mul (mul x x) x in
///         let x = 5 in
///         let y = 3 in
///         let z = add (square x) (cube y) in
///         z
///     

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
use rustyline::DefaultEditor;

fn show_help() {
    println!("Alabasta: a λ-expr REPL.");
    println!("Commands:");
    println!("    :multiline, :m - enable multiline input");
    println!("    :exit, :quit, :q, :x - exit the REPL");
    println!("    :help, :h - print this help message");
    println!("    :reference, :r - print reference");
}

fn show_reference() {
    println!(
   r"+---------------------------------------------------+
    |                 Alabasta Reference                |
    +---------------------------------------------------+
    1. Lambda Abstractions:
       - Use the pattern: \<variable>.<expression>
       - Example: \x.x + 1
    
    2. Arithmetic Operations:
       - Supported operators: +, -, *, /, %
       - Example: (3 + 5) * 2
    
    3. Let Expressions:
       - Use the pattern: let <variable> = <expression> in <scope_expression>
       - Example: let double = \x.x * 2 in double 5
    
    4. Lambda Application:
       - Use the pattern: <lambda_expression> <argument>
       - Example: (\x.x + 1) 5
    
    5. Special Notes:
       - Variables must start with a letter and can include alphanumeric characters and underscores.
       - Parentheses can be used to specify evaluation order.
       - Expressions should be separated by whitespace.
    
    6. Examples:
       - Example 1: (\x.\y.x + y) 5 10    (Applies lambda function to arguments)
       - Example 2: let square = \x.x * x in square 5    (Using let expressions)
       - Example 3: let add = \x. \y. x + y in     (Using multiline expressions)
                    let sub = \x. \y. x - y in
                    let mul = \x. \y. x * y in
                    let square = \x. mul x x in
                    let cube = \x. mul (mul x x) x in
                    let x = 5 in
                    let y = 3 in
                    let z = add (square x) (cube y) in
                    z
    
    Happy experimenting with Alabasta! Type ':quit' to exit the REPL.
    ");
}

fn show_welcome() {
    println!(r"
+-----------------------------------------------------------------------+
    █████╗ ██╗      █████╗ ██████╗  █████╗ ███████╗████████╗ █████╗ 
   ██╔══██╗██║     ██╔══██╗██╔══██╗██╔══██╗██╔════╝╚══██╔══╝██╔══██╗
   ███████║██║     ███████║██████╔╝███████║███████╗   ██║   ███████║
   ██╔══██║██║     ██╔══██║██╔══██╗██╔══██║╚════██║   ██║   ██╔══██║
   ██║  ██║███████╗██║  ██║██████╔╝██║  ██║███████║   ██║   ██║  ██║
   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═════╝ ╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝
+-----------------------------------------------------------------------+
                        A λ-calculus REPL
+-----------------------------------------------------------------------+

    Type :help or :h for help and :reference or :r for reference.

");
}

fn main() -> rustyline::Result<()>{

    show_welcome();

    let mut prompt = "λ-expr >> ".to_string();
    let mut multiline = false;
    let mut lambda_expression = String::new();

    // Initialise rustyline
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    // REPL loop
    loop {
        // Set prompt
        if multiline == false || lambda_expression.len() == 0 {
            prompt = "λ-expr >> ".to_string();
        } else {
            prompt = "+ > ".to_string();
        }

        // Read input
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {                
                // check for builtin commands or evaluate expression
                match line {
                    // exit the REPL
                    ref s if s == ":exit" || s == ":quit" || s == ":q" || s == ":x" => {
                        println!("Terminating...");
                        break
                    },
                    // enable/disable multiline input
                    ref s if s == ":multiline" || s == ":m" => {
                        multiline = !multiline;
                        
                        println!("Multiline input {}.", if multiline { "enabled" } else { "disabled" });

                        continue
                    },
                    // print help message
                    ref s if s == ":help" || s == ":h" => { 
                        show_help();
                        continue
                    },
                    ref s if s == ":reference" || s == ":r" => {
                        show_reference();
                        continue
                    },
                    _ => { }
                }

                // if multiline is disabled, evaluate the input
                if multiline == false {
                    // add input to command history
                    let _ = rl.add_history_entry(line.as_str());

                    // evaluate lambda expression
                    if let Err(err) = eval(&line.to_string()) {
                        println!("Error: {}", err);
                    }
                } else {
                    // if line is empty, evaluate the lambda expression
                    if line.len() == 0 {
                        let _ = rl.add_history_entry(lambda_expression.as_str());
                        if let Err(err) = eval(&&lambda_expression.to_string()) {
                            println!("Error: {}", err);
                        }

                        // reset the current lambda expression
                        lambda_expression = String::new();
                    } else {
                        // concatenate the current line to the lambda expression
                        lambda_expression.push_str(line.as_str());
                    }
                }
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
fn eval(lambda_expression: &String) -> Result<NormalExpressionNode, String> {
    // Generate token list from input string
    let token_list = Lexer::new(lambda_expression.to_string())
        .scan()?;

    // Parse token list into abstract syntax tree
    let ast = Parser::new(token_list)
        .parse()?;

    // Pretty print the parsed input
    println!("Parsed λ-expr :");
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
