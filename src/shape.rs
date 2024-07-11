use std::io::Write;

use screen_13::prelude::*;

use crate::registry::{BufferIndex, BufferOffset, Registry};

pub trait Shape {
    fn intersection(&self) -> Option<&'static [u32]>;
    fn compute_surface_interaction(&self) -> &'static [u32];
    fn register(&self, registry: &mut Registry) -> (BufferIndex, BufferOffset);
}
