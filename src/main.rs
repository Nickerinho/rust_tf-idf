use std::str::FromStr;
use rust_tf_idf::{tf_idf, log::Log_Level};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    log: String
}

fn main(){
    let args = Cli::parse();
    
    // ugly workaround, because i couldnt get ArgsEnum of Clap to work (retry later)
    let level = Log_Level::from_str(&args.log).unwrap();
    tf_idf::run(args.path, level);
}