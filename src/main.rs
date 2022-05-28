use make_a_lisp_in_rust::parser::Parser;
use make_a_lisp_in_rust::tokenizer::Tokenizer;
use std::io;
use std::io::Write;

fn main() -> Result<(), io::Error> {
    println!("Welcome to the super pretty lisp interpreter.");

    loop {
        print!("user> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        match &input as &str {
            "EOF" => break,
            _ => {
                let mut tokenizer = Tokenizer::new(input);
                let parser = Parser::new(&mut tokenizer);
                for ast in parser {
                    println!("{:?}", ast);
                }
            }
        }
    }

    Ok(())
}
