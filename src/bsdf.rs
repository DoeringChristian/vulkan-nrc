use std::io::Write;

use screen_13::prelude::*;

pub trait BSDF {
    fn eval(&self) -> Option<&'static str>;
    fn sample(&self) -> Option<&'static str>;
    fn write_data(&self, writer: &dyn Write) -> std::io::Result<()>;
}
