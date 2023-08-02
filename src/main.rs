mod lexer;
mod parser;
mod ast;
mod alpha;
mod beta;
mod pretty;

use lexer::Lexer;
use parser::Parser;
use alpha::AlphaConverter;
use beta::BetaReducer;
use pretty::{
    pretty_print_normal,
    pretty_print
};

fn main() {
    //let expression = r"(\x. (\z. z x) x) ((\y. y) 1 + 1)";
    //let expression = r"(\z. (\y. (\x. x y) y z)) 1";
    // let expression = r"(\x. \y. \z. x y z) 1 2 3";
    // let expression = r"(\x. \y. \z. x y z) 1 2 3";
    // let expression = "x y z";
    // let expression = r"(\f. \x. f (f x)) (\y. y * 2) 3";
    // let expression = r"(\x. x + 3) ((\y. y * 2) 5) + (\z. z / 2) (4 - 1)";
    let expression = r"let double = \x. x * 2 in double 5";

    // let expression = r"let f x = g x in
    //                    let g x = f x in
    //                    let x = 1 in
    //                    f (g (f (g x)))";

    //let expression = r"(\f. x)";
    
    // Generate token list from input string
    let token_list = Lexer::new(expression.to_string())
        .scan()
        .unwrap();

    // Parse token list into abstract syntax tree
    let ast = Parser::new(token_list)
        .parse()
        .unwrap();

    // Pretty print the parsed input
    println!("\nInput:");
    pretty_print(&ast);
    println!();

    // Perform alpha conversion on the abstract syntax tree
    let _ = AlphaConverter::new()
        .convert(&ast);

    // Print the normal form
    println!("\nAlpha Conversion form:");
    pretty_print(&ast);
    println!();

    // Perform beta reduction on the abstract syntax tree
    let mut beta_reducer = BetaReducer::new();
    let normal_form = beta_reducer.convert(&ast);
    let result = beta_reducer.reduce(&normal_form);

    // Print the normal form
    println!("\nNormal form (after Beta-reduction):");
    pretty_print_normal(&result);
    println!();
}