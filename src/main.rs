pub mod loader;

fn main() {
    println!("Hello, world!");
    loader::load("./test.class".to_string());
}
