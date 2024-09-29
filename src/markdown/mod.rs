use pulldown_cmark::{Options, Parser};
use pulldown_cmark::html::push_html;
use anyhow::Result;
pub mod core;

pub fn parser(content: &str) -> Result<String> {
  let options = Options::all();
  let parser = Parser::new_ext(content,options);
  let parser = core::custom_code_parser::CustomCodeParser::new(parser);
  let mut html = String::with_capacity(content.len() * 4);
  push_html(&mut html, parser);
  Ok(html)
}


#[cfg(test)]
mod tests {
  use std::fs;
  use crate::markdown::parser;

  #[test]
  fn parser_test() {
    let content = fs::read_to_string(r"C:\Users\26216\code\github\ttdly.github.io\posts\5.md").unwrap_or_else(|e| panic!("{}", e));
    let html = parser(&content).unwrap_or_else(|e| panic!("{}", e));
    println!("{}", html);
  }
}