use std::io::Write;

use screen_13::prelude::*;

use crate::registry::{Register, Registry};

pub trait Shape: Register {
    fn intersection(&self) -> Option<&'static str>;
    fn compute_surface_interaction(&self) -> Option<&'static str>;
}
