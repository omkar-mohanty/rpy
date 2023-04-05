use std::path::PathBuf;

use clap::Parser;
use rpy::Result;
use rpy::parser as rpy_parser;            

#[derive(Parser, Debug)]
struct Args {
    source_file: PathBuf,
    output_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let source = std::fs::read_to_string(args.source_file)?;

    println!("{}", source);

    let ast = rpy_parser::statements(&source)?;

    println!("{}", ast.len());

    for node in ast {
        println!("{}", node)
    }

    Ok(())
}
