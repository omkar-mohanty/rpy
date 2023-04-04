use std::path::PathBuf;

use clap::Parser;
use rpy::Result;

#[derive(Parser, Debug)]
struct Args {
    source_file: PathBuf,
    output_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    Ok(())
}
