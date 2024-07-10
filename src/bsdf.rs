use std::io::Write;

use screen_13::prelude::*;

use crate::registry::Register;

pub trait BSDF: Register {
    fn eval(&self) -> &'static [u32];
    fn sample(&self) -> &'static [u32];
}
