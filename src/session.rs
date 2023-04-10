use std::{path::PathBuf, fs::{OpenOptions, File}, str::FromStr, io::Write, mem::size_of};

use crate::{jit::JIT, Result};

pub struct Sesssion {
    source: String,
    jit: JIT,
    output_file: Option<PathBuf>,
}

impl Sesssion {
    pub fn new(source: String, output_file: Option<PathBuf>) -> Self {
        Self {
            source,
            jit: JIT::default(),
            output_file,
        }
    }

    pub fn compile(mut self) -> Result<()> {
        let machine_code = self.jit.compile(&self.source)?;

        let default_path = PathBuf::from_str("a.out").unwrap();
        let path = self.output_file.unwrap_or(default_path);

        let file = File::options().write(true).open(path)?;

        Ok(())
    }
}
