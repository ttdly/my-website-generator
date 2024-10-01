use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{append_highlighted_html_for_styled_line, IncludeBackground};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use anyhow::Result;

use super::lang_transform;

pub fn highlight(code: &str, lang: &str) -> Result<String>{
  let ps = SyntaxSet::load_defaults_newlines();
  let ts = ThemeSet::load_defaults();
  let theme = &ts.themes["InspiredGitHub"];
  let index_lang = lang_transform::transform(lang);
  let syntax = match ps.find_syntax_by_name(index_lang) {
    Some(syntax) => syntax,
    None => match ps.find_syntax_by_extension(index_lang) {
      Some(syntax) => syntax,
      None => ps.find_syntax_by_extension("txt").unwrap()
    }
  };
  let mut highlighter = HighlightLines::new(syntax, theme);
  let mut output = String::from(
    format!("<div class=\"language-{}\"><button class=\"copy\"></button><pre><code>", lang));
  for line in LinesWithEndings::from(code) {
    let regions = highlighter.highlight_line(line, &ps)?;
    output.push_str("<span class=\"line\">");
    append_highlighted_html_for_styled_line(
      &regions[..],
      IncludeBackground::No,
      &mut output,
    )?;
    output.push_str("</span>")
  }
  output.push_str("</code></pre></div>\n");
  Ok(output)
}


#[cfg(test)]
mod test{
  use super::*;
  #[test]
  fn test_my_highlight(){
    let result = highlight("let a = 1 + 2;\nconsole.log(a)", "js")
      .unwrap_or_else(|_| String::from("error"));
    print!("{}", result)
  }
}