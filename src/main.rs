use std::env;
pub mod markdown;
pub mod page;

fn main() {
    let args: Vec<String> = env::args().collect();
    print!("{}", args.len());
    for i in args{
        println!("{}", i);
    }
}
