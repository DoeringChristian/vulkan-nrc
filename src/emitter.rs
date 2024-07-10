use crate::registry::Register;

pub trait Emitter: Register {
    fn eval(&self) -> &'static [u32];
    fn sample(&self) -> &'static [u32];
}
