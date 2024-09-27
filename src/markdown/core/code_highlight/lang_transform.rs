pub fn transform(from_str: &str) -> &str{
  let from_str_lower = from_str.to_lowercase();
  let from_str_lower = from_str_lower.as_str();
  match from_str_lower {
    "javascript" | "js" => "JavaScript",
    "csharp" | "cs" | "c#" => "C#",
    "c++" => "C++",
    "java" => "Java",
    "html" => "HTML",
    "yaml" => "YAML",
    "rust" | "rs" => "Rust",
    "shell" => "bash",
    "python" | "py" => "Python",
    _ => from_str
  }
}

#[cfg(test)]
mod test{
  use syntect::parsing::SyntaxSet;
  #[test]
  fn traverse_syntax_set(){
    let ps = SyntaxSet::load_defaults_newlines();
    for syntax in ps.syntaxes(){
      println!("{:?}", syntax.name)
    }
  }
}