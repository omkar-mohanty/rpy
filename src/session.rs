use crate::{jit::JIT, Expr};

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
}
