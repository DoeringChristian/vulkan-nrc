use crevice::{glsl::GlslStruct, std140::AsStd140};
use glam::*;

use crate::registry::{BufferIndex, BufferOffset, CallableIndex, IntersectionIndex};

pub struct Instance {
    pub to_world: Mat4,
    pub shape: usize,
    pub bsdf: usize,
    pub emitter: usize,
}

#[repr(C)]
#[derive(AsStd140, GlslStruct, Debug, Clone, Copy)]
pub struct InstanceData {
    pub to_world: Mat4,
    pub shape_buf: BufferIndex,
    pub shape_offset: BufferOffset,
    pub intersection: IntersectionIndex,
    pub compute_surface_interaction: CallableIndex,
    pub bsdf_buf: BufferIndex,
    pub bsdf_offset: BufferOffset,
    pub bsdf_eval: CallableIndex,
    pub bsdf_sample: CallableIndex,
    pub emitter_buf: BufferIndex,
    pub emitter_offset: BufferOffset,
    pub emitter_eval: CallableIndex,
    pub emitter_sample: CallableIndex,
}
