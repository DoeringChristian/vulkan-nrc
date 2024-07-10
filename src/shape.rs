use std::io::Write;

use screen_13::prelude::*;

use crate::registry::{Register, Registry};

pub trait Shape: Register {
    fn intersection(&self) -> Option<&'static [u32]>;
    fn compute_surface_interaction(&self) -> &'static [u32];
}
