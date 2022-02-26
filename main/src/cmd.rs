
pub mod cmd {
  pub enum Token {
    Exit,
  }

  pub fn parser (inp: &str) -> Result<Token, String> {
    // let i = String::from(inp);
    // println!("{}", &i);

    // println!("{}", inp == "Exit");

    match inp {
      "Exit" => {
        Ok(Token::Exit)
      },
      _ => {
        Err(String::from(inp))
      }
    }
  }
}