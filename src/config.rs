use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_insensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Self, &str> {
      if args.len() < 3 {
          return Err("Not enough arguments")
      }

      let query = args[1].clone();
      let filename = args[2].clone();
      let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

      Ok(Self { query, filename, case_insensitive })
  }
}