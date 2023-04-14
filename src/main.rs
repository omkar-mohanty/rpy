use core::mem;
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

    unsafe { run_code::<(), ()>(source, ())? };

    Ok(())
}

unsafe fn run_code<I, O>(source: String, input: I) -> Result<O> {
    let session = Sesssion::new(source);

    // Pass the string to the JIT, and it returns a raw pointer to machine code.
    let code_ptr = session.compile()?;
    // Cast the raw pointer to a typed function pointer. This is unsafe, because
    // this is the critical point where you have to trust that the generated code
    // is safe to be called.
    let code_fn = mem::transmute::<_, fn(I) -> O>(code_ptr);
    // And now we can call it!
    Ok(code_fn(input))
}
