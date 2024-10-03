use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use gray_matter::engine::YAML;
use gray_matter::{Matter, Pod};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

pub fn from_action(github_event_file_path: &str,
               save_markdown_file_path: &str,
               save_html_file_path: &str) -> Result<()> {
  let json_content = fs::read_to_string(&github_event_file_path)?;
  let json_object: Value = serde_json::from_str(&json_content)?;
  let discussion_body = match json_object["discussion"]["body"].as_str(){
    Some(body) => body,
    None => return Err(anyhow!("body is null")),
  };
  let discussion_number = match json_object["discussion"]["number"].as_u64(){
    Some(number) => number,
    None => return Err(anyhow!("number is null")),
  };
  println!("Handling {}...", discussion_number);
  save2md(discussion_body,&discussion_number,&json_object, &save_markdown_file_path)?;
  save2html(discussion_body, &discussion_number, &json_object, &save_html_file_path)?;
  println!("Done!");
  Ok(())
}

fn save2md(discussion_body: &str, discussion_number: &u64,
           json_object: &Value, save_markdown_file_path: &str) -> Result<()>{
  let front_matter_string = create_front_matter_string(&json_object)?;
  let markdown_body = format!("{}{}", front_matter_string, discussion_body);
  let save_markdown_file_path = PathBuf::from(save_markdown_file_path);
  let save_markdown_file_path =
    save_markdown_file_path.join(format!("{}.md", discussion_number));
  println!("Saving markdown file to {:?}", save_markdown_file_path);
  fs::write(&save_markdown_file_path, markdown_body)?;
  Ok(())
}

fn create_front_matter_string(json_object: &Value) -> Result<String>{
  let mut front_matter_string = String::from("---\n");
  match json_object["discussion"]["title"].as_str(){
    Some(title) => front_matter_string.push_str(&format!("title: {}\n", title)),
    None => ()
  }
  match json_object["discussion"]["created_at"].as_str(){
    Some(create_at) =>
      front_matter_string.push_str(&format!("create_at: {}\n", create_at)),
    None => ()
  }
  match json_object["discussion"]["updated_at"].as_str(){
    Some(update_at) =>
      front_matter_string.push_str(&format!("updated_at: {}\n", update_at)),
    None => ()
  }
  match json_object["discussion"]["comments"].as_u64(){
    Some(comments) =>
      front_matter_string.push_str(&format!("comments: {}\n", comments)),
    None => ()
  }
  match json_object["discussion"]["locked"].as_bool(){
    Some(locked) =>
      front_matter_string.push_str(&format!("locked: {}\n", locked)),
    None => ()
  }
  match json_object["discussion"]["labels"].as_array() {
    Some(labels) => {
      let mut labels_yaml = String::new();
      for label in labels {
        labels_yaml.push_str(&format!("\n  -{}", label["name"].as_str().unwrap()));
      }
      front_matter_string.push_str(&format!("labels: {}\n", labels_yaml));
    }
    None => ()
  }
  front_matter_string.push_str("---\n\n");
  Ok(front_matter_string)
}

fn save2html(discussion_body: &str, discussion_number: &u64,
             json_object: &Value, save_html_file_path: &str) -> Result<()> {
  let discussion_html = crate::markdown::parser(&discussion_body)?;
  let mut context = create_context_from_action(json_object)?;
  context.insert("blog_body", &discussion_html);
  render_html(save_html_file_path, context, discussion_number.to_string().as_str())?;
  Ok(())
}

fn render_html(save_html_file_path: &str, context: Context, save_html_file_name: &str) -> Result<()> {
  let tera = match Tera::new("resources/templates/**/*.html") {
    Ok(tera) => tera,
    Err(e) => return Err(anyhow!("{}", e)),
  };
  let html = tera.render("post.html", &context)?;
  let save_html_file_path = PathBuf::from(save_html_file_path)
    .join(format!("{}.html", save_html_file_name));
  println!("Saving html file to {:?}", save_html_file_path);
  fs::write(save_html_file_path, html)?;
  Ok(())
}

fn create_context_from_action(json_object: &Value) -> Result<Context> {
  let mut context = Context::new();
  match json_object["discussion"]["title"].as_str(){
    Some(title) => context.insert("title", title),
    None => ()
  }
  match json_object["discussion"]["created_at"].as_str(){
    Some(create_at) =>
      context.insert("create_at", &parse_date_str(String::from(create_at))?),
    None => ()
  }
  match json_object["discussion"]["updated_at"].as_str(){
    Some(update_at) =>
      context.insert("update_at", &parse_date_str(String::from(update_at))?),
    None => ()
  }
  match json_object["discussion"]["comments"].as_u64() {
    Some(comments) =>
      context.insert("comments", &comments),
    None => ()
  }
  match json_object["discussion"]["labels"].as_array() {
    Some(labels) => {
      let labels_str:Vec<&str> = labels.iter()
        .map( |label_obj| label_obj["name"].as_str().unwrap_or_else(|| "unknown"))
        .collect::<Vec<&str>>();
      context.insert("labels", &labels_str);
    }
    None => ()
  }
  Ok(context)
}

pub fn from_markdown_file(markdown_file_path: &str, save_html_file_path: &str) -> Result<()> {
  let markdown_content = fs::read_to_string(markdown_file_path)?;
  let number = match Path::new(markdown_file_path).file_stem(){
    Some(number) => number,
    None => return Err(anyhow!("can not find markdown file")),
  };
  let number = match  number.to_str(){
    Some(number) => number,
    None => return Err(anyhow!("can not find markdown file")),
  };
  let markdown_content_html = crate::markdown::parser(&markdown_content)?;
  let matter = Matter::<YAML>::new();
  let result = matter.parse(&markdown_content);
  let data = match result.data.as_ref() {
    Some(data) => data,
    None => return Err(anyhow!("gray matter parse data is null")),
  };
  let data = data.as_hashmap()?;
  let locked = data.get("locked").unwrap_or(&Pod::Boolean(false)).as_bool()?;
  if locked { return Ok(()) }
  let mut context = create_context_from_md(data)?;
  context.insert("blog_body", &markdown_content_html);
  render_html(save_html_file_path, context, number)
}

pub fn from_markdown_dir(markdown_dir_path: &str, save_html_file_path: &str) -> Result<()> {
  for entry in fs::read_dir(markdown_dir_path)? {
    let entry = entry?;
    let path = entry.path();
    let path = match path.as_os_str().to_str() {
      Some(path) => path,
      None => return Err(anyhow!("path can not convert to str")),
    };
    from_markdown_file(&path, save_html_file_path)?
  };
  Ok(())
}

fn create_context_from_md(data: HashMap<String, Pod>) -> Result<Context> {
  let mut context = Context::new();
  match data.get("title") {
    Some(title) => context.insert("title", &title.as_string()?),
    None => ()
  }
  match data.get("create_at") {
    Some(create_at) =>
      context.insert("create_at", &parse_date_str(create_at.as_string()?)?),
    None => ()
  }
  match data.get("comments") {
    Some(comments) => context.insert("comments", &comments.as_i64()?),
    None => ()
  }
  match data.get("update_at") {
    Some(update_at) =>
      context.insert("update_at", &parse_date_str(update_at.as_string()?)?),
    None => ()
  }
  match data.get("labels") {
    Some(labels) => {
      let labels = labels.as_vec()?;
      let labels_str:Vec<String> = labels.iter()
        .map(|x| x.as_string().unwrap_or_else(|e| e.to_string()))
        .collect::<Vec<String>>();
      context.insert("labels", &labels_str);
    },
    None => ()
  };
  Ok(context)
}
fn parse_date_str(string: String) -> Result<String> {
  Ok(DateTime::parse_from_rfc3339(&string)?.with_timezone(&Local).format("%Y年%m月%d日").to_string())
}
#[cfg(test)]
mod tests {
  use crate::page::render::{from_action, from_markdown_dir, from_markdown_file};
  use fs_extra::copy_items;
  use fs_extra::dir::CopyOptions;

  #[test]
  fn test_from_action() {
    from_action(r"C:\Users\26216\Desktop\git.txt", r"D:\Code\web\test_page", r"D:\Code\web\test_page")
      .expect("TODO: panic message");
    let mut options = CopyOptions::new(); // Initialize default values for CopyOptions
    options.copy_inside = true;
    options.overwrite = true;
    copy_items(&["resources/templates/assets"], r"D:\Code\web\test_page", &options)
      .expect("aaa");
  }
  #[test]
  fn test_from_md(){
    from_markdown_file(r"C:\Users\26216\code\github\ttdly.github.io\posts\12.md",
    r"D:\Code\web\test_page").expect("aaaa");
    let mut options = CopyOptions::new(); // Initialize default values for CopyOptions
    options.copy_inside = true;
    options.overwrite = true;
    copy_items(&["resources/templates/assets"], r"D:\Code\web\test_page", &options)
      .expect("aaa");
  }
  #[test]
  fn test_from_md_dir(){
    from_markdown_dir(r"D:\Code\md",
                       r"D:\Code\web\test_page").expect("aaaa");
    let mut options = CopyOptions::new(); // Initialize default values for CopyOptions
    options.copy_inside = true;
    options.overwrite = true;
    copy_items(&["resources/templates/assets"], r"D:\Code\web\test_page", &options)
      .expect("aaa");
  }
}