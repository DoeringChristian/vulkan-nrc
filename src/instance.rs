use crevice::{glsl::GlslStruct, std140::AsStd140};
use glam::*;

use crate::registry::{BufferIndex, CallableIndex};

pub struct Instance {
    pub to_world: Mat4,
    pub shape: usize,
    pub material: usize,
    pub emitter: usize,
}

#[derive(AsStd140, GlslStruct, Debug, Clone, Copy)]
pub struct InstanceData {
    pub to_world: Mat4,
    pub shape_buf: BufferIndex,
    pub compute_surface_interaction: CallableIndex,
}
