use std::io::Write;
use std::io::stdin;
use std::io::stdout;
mod cmd;

fn main() {
    loop {
        let mut inp = String::new();
        stdout().write("Your Mom DB>\n".as_bytes()).unwrap();
        stdin().read_line(&mut inp).ok().expect("Failed to read line");
        inp.pop();
        
        let res = cmd::cmd::parser(&inp);
        match res {
            Ok(cmd::cmd::Token::Exit) => {
                println!("Exiting...");
                break;
            },
            Err(c) => {
                println!("Unrecognized command: {}", c);
            }
        }
    }
    // println!("{}", inp);
}
