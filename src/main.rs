mod lexer;
mod parser;
mod ast;
mod alpha;

use lexer::Lexer;
use parser::Parser;
use ast::ExpressionNode;
use alpha::AlphaConverter;

fn main() {
    //let expression = "x y z";
    // let expression = r"(\f. \x. f (f x)) (\y. y * 2) 3";
    let expression = r"(\x. x + 3) ((\y. y * 2) 5) + (\z. z / 2) (4 - 1)";
    // let expression = r"let double = \x. x * 2 in double 5";

    // let expression = r"let f x = g x in
    //                    let g x = f x in
    //                    let x = 1 in
    //                    f (g (f (g x)))";

    //let expression = r"(\f. x)";
    
    let mut lexer = Lexer::new(expression.to_string());
    let token_list = lexer.scan().unwrap();
    let ast = Parser::new(token_list).parse().unwrap();
    let _ = AlphaConverter::new().convert(&ast);

    println!("\n{:?}", ast);
}