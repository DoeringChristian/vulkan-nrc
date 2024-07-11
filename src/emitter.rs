use crate::registry::{BufferIndex, BufferOffset, Registry};

pub trait Emitter {
    fn eval(&self) -> &'static [u32];
    fn sample(&self) -> &'static [u32];
    fn register(&self, registry: &mut Registry) -> (BufferIndex, BufferOffset);
}
