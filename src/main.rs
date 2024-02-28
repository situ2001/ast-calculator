use std::io::{self, Write};

use ast_calculator::{evaluate, lexer, parser};

fn main() {
    println!("Welcome to the AST calculator! Just type in an expression and press enter.");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let expression = parser.parse();
        // dbg!(&expression);

        let result = evaluate::eval(expression);
        println!("{}", result);
    }
}
