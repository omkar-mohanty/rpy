use std::path::PathBuf;

use clap::Parser;
use rpy::{Result, Sesssion};

#[derive(Parser, Debug)]
struct Args {
    source_file: PathBuf,
    output_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let source = std::fs::read_to_string(args.source_file)?;

    let session = Sesssion::new(source);

    Ok(())
}
