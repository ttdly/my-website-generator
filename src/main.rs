use std::env;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use crate::page::render;

pub mod markdown;
pub mod page;

const SAVE_MARKDOWN_PATH: &str = "resources/markdown";
const SAVE_HTML_PATH: &str = "dist";
const STATIC_FILES_PATH: &str = "resources/templates/assets";
fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "action" => action(),
        "recompile" => recompile(),
        _ => println!("Not implemented yet")
    }
    post_render();
    move_static_files();
}

fn action() {
    if let Some(val) = env::var_os("GITHUB_EVENT_PATH") {
        match render::from_action(val.to_str().unwrap(), SAVE_MARKDOWN_PATH, SAVE_HTML_PATH) {
            Ok(_) => {}
            Err(err) => println!("An error occured: {}", err)
        }
    } else {
        println!("环境变量 MY_ENV_VARIABLE 不存在");
    }
}

fn recompile() {
    match render::from_markdown_dir(SAVE_MARKDOWN_PATH, SAVE_HTML_PATH) {
        Ok(_) => {}
        Err(err) => println!("An error occured: {}", err)
    }
}

fn post_render(){
    // 保存 md 数据
    match render::render_post_list(SAVE_HTML_PATH) {
        Ok(_) => {}
        Err(err) => println!("An error occured: {}", err)
    } ;
}

fn move_static_files() {
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;
    match copy_items(&[STATIC_FILES_PATH], SAVE_HTML_PATH, &options){
        Ok(_)=> {}
        Err(err) => println!("An error occured: {}", err)
    }
}