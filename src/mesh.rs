use std::sync::Arc;

use crevice::{glsl::GlslStruct, std140::AsStd140};
use glam::*;

use crate::registry::{BufferIndex, Register, Registry};
use crate::shape::Shape;

use screen_13::prelude::*;

pub struct Mesh {
    indices_count: u32,
    indices: Arc<Buffer>,
    positions: Arc<Buffer>,
    normals: Arc<Buffer>,
    uvs: Arc<Buffer>,
}

impl Register for Mesh {
    fn register(&self, registry: &mut Registry) -> BufferIndex {
        let data = MeshData {
            indices: registry.add_buffer(&self.indices),
            indices_count: self.indices_count,
            positions: registry.add_buffer(&self.positions),
            normals: registry.add_buffer(&self.normals),
            uvs: registry.add_buffer(&self.uvs),
        };
        registry.upload_std140(data)
    }
}

impl Shape for Mesh {
    fn intersection(&self) -> Option<&'static [u32]> {
        None
    }

    fn compute_surface_interaction(&self) -> &'static [u32] {
        todo!()
    }
}

#[derive(AsStd140, GlslStruct, Debug, Clone, Copy)]
pub struct MeshData {
    pub indices: BufferIndex,
    pub indices_count: u32,
    pub positions: BufferIndex,
    pub normals: BufferIndex,
    pub uvs: BufferIndex,
}
