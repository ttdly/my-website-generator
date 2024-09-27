use pulldown_cmark::{CodeBlockKind, Event, Tag};
use super::code_highlight;

pub struct CustomCodeParser<'parser_event, T: Iterator<Item=Event<'parser_event>>> {
  iterator: T,
}

impl<'parser_event, T: Iterator<Item=Event<'parser_event>>> CustomCodeParser<'parser_event, T> {
  pub fn new(iterator: T) -> Self {
    Self {
      iterator
    }
  }
}

impl<'parser_event, T: Iterator<Item=Event<'parser_event>>> Iterator
for CustomCodeParser<'parser_event, T> {
  type Item = Event<'parser_event>;

  fn next(&mut self) -> Option<Self::Item> {
    let language = match self.iterator.next() {
      Some(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(language)))) => language,
      other_event => return other_event,
    };
    let mut code = String::new();
    while let Some(event) = self.iterator.next() {
      match event {
        Event::Text(line) => code.push_str(&line.into_string()),
        _ => break,
      }
    }
    let code_html =
      code_highlight::parser::highlight(&code,&language).unwrap_or_else(|e| e.to_string());
    Some(Event::Html(code_html.into()))
  }
}