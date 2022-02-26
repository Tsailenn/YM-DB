use std::io::Write;
use std::io::stdin;
use std::io::stdout;
pub mod commands;
use commands::token::Token;

fn main() {
    loop {
        let mut inp = String::new();
        stdout().write("Your Mom DB>\n".as_bytes()).unwrap();
        stdin().read_line(&mut inp).ok().expect("Failed to read line");
        inp.pop();
        
        let res = commands::parser(&inp);
        match res {
            Ok(Token::Exit) => {
                println!("Exiting...");
                break;
            },
            Ok(Token::Insert(arg)) => {
                println!("Insert...");
                println!("{}", arg);
            },
            Ok(Token::Delete(arg)) => {
                println!("Delete...");
                println!("{}", arg);
            },
            Ok(Token::Select(arg)) => {
                println!("Select...");
                println!("{}", arg);
            },
            Ok(Token::Cast(arg)) => {
                println!("Cast...");
                println!("{}", arg);
            }
            Err(c) => {
                println!("Unrecognized command: {}", c);
            }
        };
    }
    // println!("{}", inp);
}
