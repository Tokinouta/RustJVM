pub mod loader;

// use clap to handle command line arguments
use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(version)]
struct Cmd {
    #[arg(short, long)]
    classpath: String,
    class: String,
    args: Vec<String>,
}

fn main() {
    println!("Hello, world!");
    // loader::load("./test.class".to_string());
    let cmd = Cmd::parse();
    println!("{:?}", cmd);
}
