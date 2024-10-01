use std::fs;
use std::path::PathBuf;
use anyhow::{anyhow, Result};
use serde_json::Value;
use tera::{Context, Tera};

pub fn convert(github_event_file_path: &str,
               save_markdown_file_path: &str,
               save_html_file_path: &str) -> Result<()> {
  let json_content = fs::read_to_string(&github_event_file_path)?;
  let json_object: Value = serde_json::from_str(&json_content)?;
  save2md(&json_object, &save_markdown_file_path)?;
  save2html(&json_object, &save_html_file_path)?;
  Ok(())
}

fn save2md(json_object: &Value, save_markdown_file_path: &str) -> Result<()>{
  let discussion_body = match json_object["discussion"]["body"].as_str(){
    Some(body) => body,
    None => return Err(anyhow!("body is null")),
  };
  let discussion_number = match json_object["discussion"]["number"].as_u64(){
    Some(number) => number,
    None => return Err(anyhow!("number is null")),
  };
  let front_matter_string = create_front_matter_string(&json_object)?;
  let markdown_body = format!("{}{}", front_matter_string, discussion_body);
  let save_markdown_file_path = PathBuf::from(save_markdown_file_path);
  let save_markdown_file_path =
    save_markdown_file_path.join(format!("{}.md", discussion_number));
  println!("Save to {:?}....", &save_markdown_file_path);
  fs::write(&save_markdown_file_path, markdown_body)?;
  println!("Success!");
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

fn save2html(json_object: &Value, save_html_file_path: &str) -> Result<()> {
  let tera = match Tera::new("resources/templates/**/*.html") {
    Ok(tera) => tera,
    Err(e) => return Err(anyhow!("{}", e)),
  };
  let discussion_body = match json_object["discussion"]["body"].as_str(){
    Some(body) => body,
    None => return Err(anyhow!("body is null")),
  };
  let discussion_number = match json_object["discussion"]["number"].as_u64(){
    Some(number) => number,
    None => return Err(anyhow!("number is null")),
  };
  let discussion_html = crate::markdown::parser(&discussion_body)?;
  let mut context = create_context(json_object)?;
  context.insert("blog_body", &discussion_html);
  let html = tera.render("post.html", &context)?;
  let save_html_file_path = PathBuf::from(save_html_file_path)
    .join(format!("{}.html", discussion_number));
  fs::write(save_html_file_path, html)?;
  Ok(())
}

fn create_context(json_object: &Value) -> Result<Context> {
  let mut context = Context::new();
  match json_object["discussion"]["title"].as_str(){
    Some(title) => context.insert("title", title),
    None => ()
  }

  // match json_object["discussion"]["created_at"].as_str(){
  //   Some(create_at) =>
  //     front_matter_string.push_str(&format!("create_at: {}\n", create_at)),
  //   None => ()
  // }
  // match json_object["discussion"]["updated_at"].as_str(){
  //   Some(update_at) =>
  //     front_matter_string.push_str(&format!("updated_at: {}\n", update_at)),
  //   None => ()
  // }
  // match json_object["discussion"]["comments"].as_u64(){
  //   Some(comments) =>
  //     front_matter_string.push_str(&format!("comments: {}\n", comments)),
  //   None => ()
  // }
  // match json_object["discussion"]["locked"].as_bool(){
  //   Some(locked) =>
  //     front_matter_string.push_str(&format!("locked: {}\n", locked)),
  //   None => ()
  // }
  // match json_object["discussion"]["labels"].as_array() {
  //   Some(labels) => {
  //     let mut labels_yaml = String::new();
  //     for label in labels {
  //       labels_yaml.push_str(&format!("\n  -{}", label["name"].as_str().unwrap()));
  //     }
  //     front_matter_string.push_str(&format!("labels: {}\n", labels_yaml));
  //   }
  //   None => ()
  // }
  Ok(context)
}

#[cfg(test)]
mod tests {
  use fs_extra::copy_items;
  use fs_extra::dir::CopyOptions;
  use crate::page::action2md_html::convert;

  #[test]
  fn it_works() {
    convert(r"C:\Users\26216\Desktop\git.txt", r"C:\Users\26216\Desktop", r"C:\Users\26216\Desktop")
      .expect("TODO: panic message");
    let mut options = CopyOptions::new(); // Initialize default values for CopyOptions
    options.copy_inside = true;
    options.overwrite = true;
    copy_items(&["resources/templates/assets"], r"C:\Users\26216\Desktop", &options)
      .expect("aaa");
  }
}