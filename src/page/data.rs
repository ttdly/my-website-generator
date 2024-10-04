use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NavLink {
  title: String,
  link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostItem {
  pub(crate) title: String,
  pub(crate) create_at: String,
  pub(crate) id: u64,
  pub(crate) post_abstract: String,
}

pub struct PostItemList{
  posts: Vec<PostItem>,
}

impl PostItem {
  pub fn new() -> Self {
    PostItem {
      title: String::new(),
      create_at: String::new(),
      id: 0,
      post_abstract: String::new(),
    }
  }
}

impl PostItemList {
  pub fn new() -> PostItemList {
    let posts_data_path = std::path::Path::new("resources/data/posts.json");
    if !posts_data_path.exists() {
      match fs::write(posts_data_path, String::from("[]")) {
        Ok(_) => (),
        Err(e) => panic!("{}", e.to_string()),
      };
    }
    let posts_data = match fs::read_to_string(posts_data_path) {
      Ok(v) => match serde_json::from_str::<Vec<PostItem>>(&v){
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string()),
      },
      Err(e) => panic!("{}", e.to_string()),
    };
    PostItemList {
      posts: posts_data,
    }
  }

  pub fn insert(mut self, mut post_item: PostItem) -> PostItemList {
    post_item.title = post_item.title.replace("\"","");
    post_item.create_at = post_item.title.replace("\"", "");
    self = self.remove_a_item(&post_item);
    self.posts.push(post_item);
    self
  }

  fn remove_a_item(mut self, item: &PostItem) -> PostItemList {
    for i in 0..self.posts.len() {
      if self.posts[i].id == item.id {
        self.posts.remove(i);
        break;
      }
    }
    self
  }

  pub fn remove(self, post_item: PostItem) -> PostItemList {
    self.remove_a_item(&post_item)
  }

  pub fn save(mut self) {
    self.posts.sort_by(|a, b| b.id.cmp(&a.id));
    let json_str = match serde_json::to_string(&self.posts) {
      Ok(v) => v,
      Err(e) => panic!("{}", e.to_string()),
    };
    match fs::write("resources/data/posts.json", json_str){
      Ok(_) => (),
      Err(e) => panic!("{}", e.to_string()),
    };
  }
}

pub fn get_nav_link() -> Vec<NavLink> {
  let json = match fs::read_to_string("resources/data/nav.json") {
    Ok(file) => file,
    Err(e) => panic!("{}", e.to_string()),
  };
  match serde_json::from_str(&json) {
    Ok(links) => links,
    Err(e) => panic!("{}", e.to_string()),
  }
}

