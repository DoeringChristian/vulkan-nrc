use crevice::{glsl::GlslStruct, std140::AsStd140};
use glam::*;

use crate::registry::{BufferIndex, CallableIndex};

pub struct Instance {
    pub to_world: Mat4,
    pub shape: usize,
    pub bsdf: usize,
    pub emitter: usize,
}

#[derive(AsStd140, GlslStruct, Debug, Clone, Copy)]
pub struct InstanceData {
    pub to_world: Mat4,
    pub shape_buf: BufferIndex,
    pub compute_surface_interaction: CallableIndex,
    pub bsdf_buf: BufferIndex,
    pub bsdf_eval: CallableIndex,
    pub bsdf_sample: CallableIndex,
    pub emitter_buf: BufferIndex,
    pub emitter_eval: CallableIndex,
    pub emitter_sample: CallableIndex,
}
