use gray_matter::{Matter, ParsedEntity};
use gray_matter::engine::YAML;
use anyhow::Result;

pub fn parser(content: &str) -> Result<ParsedEntity> {
  let matter = Matter::<YAML>::new();
  Ok(matter.parse(content))
}

#[cfg(test)]
mod tests {
  use std::fs;
  use super::*;

  #[test]
  fn parser_test() {
    let content = fs::read_to_string(r"C:\Users\26216\code\github\ttdly.github.io\posts\5.md").unwrap_or_else(|e| panic!("{}", e));
    match parser(&content) {
      Ok(html) => println!("{:#?}", html),
      Err(e) => panic!("{}", e),
    };
  }
}