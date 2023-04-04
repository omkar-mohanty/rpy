use std::{fs, path::PathBuf};

use clap::Parser;
use rpy::Result;

#[derive(Parser, Debug)]
struct Args {
    source_file: PathBuf,
    output_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = fs::read(args.source_file)?;
    let output_file = args.output_file.unwrap_or("a.out".into());
    Ok(())
}
