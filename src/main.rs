pub mod attribute;
pub mod classpath;
pub mod classfile;
pub mod loader;
pub mod runtime_data_area;

// use clap to handle command line arguments
use clap::{arg, Parser};
use classpath::ClassPath;

#[derive(Parser, Debug)]
#[command(version)]
struct Cmd {
    #[arg(short, long)]
    classpath: Option<String>,
    #[arg(long = "Xjre")]
    xjre: String,
    class: String,
    args: Vec<String>,
}

fn start_jvm(cmd: &Cmd) {
    let classpath = match &cmd.classpath {
        Some(cp) => cp.clone(),
        None => ".".to_string(),
    };
    let cp = ClassPath::new(cmd.xjre.clone(), classpath);
    println!("{:?}", cmd);
    let class_name = cmd.class.replace(".", "/");
    if let Ok(class_data) = cp.read_class(class_name.as_str()) {
        println!("{:?}", class_data);
    } else {
        println!("class not found");
    };
}

fn main() {
    println!("Hello, world!");
    // loader::load("./test.class".to_string());
    let cmd = Cmd::parse();
    println!("{:?}", cmd);

    start_jvm(&cmd);
}
