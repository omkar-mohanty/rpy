use crate::{jit::JIT, Result};

pub struct Sesssion {
    source: String,
    jit: JIT,
}

impl Sesssion {
    pub fn new(source: String) -> Self {
        Self {
            source,
            jit: JIT::default(),
        }
    }

    pub fn compile(mut self) -> Result<*const u8> {
        self.jit.compile(&self.source)
    }
}
