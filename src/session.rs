use crate::jit::JIT;

pub struct Sesssion {
    source: String,
    jit: JIT
}

impl Sesssion {
    pub fn new(source: String) -> Self {
       Self { source, jit: JIT::default() } 
    }
}
