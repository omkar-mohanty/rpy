use std::path::PathBuf;

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

    pub fn compile(&mut self) -> Result<()> {
        let machine_code = self.jit.compile(&self.source)?;
        Ok(())
    }
}
