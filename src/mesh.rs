use std::sync::Arc;

use crevice::{glsl::GlslStruct, std140::AsStd140};
use glam::*;

use crate::registry::{upload_buffer, BufferIndex, Register, Registry};
use crate::shape::Shape;

use screen_13::prelude::*;

#[derive(Clone, Copy)]
pub struct MeshDesc<'a> {
    pub indices: &'a [u32],
    pub positions: &'a [[f32; 3]],
    pub normals: &'a [[f32; 3]],
    pub uvs: &'a [[f32; 2]],
}

pub struct Mesh {
    pub indices_count: u32,
    pub indices: Arc<Buffer>,
    pub positions: Arc<Buffer>,
    pub normals: Arc<Buffer>,
    pub uvs: Arc<Buffer>,
}

impl Mesh {
    pub fn new(
        desc: MeshDesc,
        rgraph: &mut RenderGraph,
        cache: &mut HashPool,
        device: &Arc<Device>,
    ) -> Self {
        let indices = upload_buffer(bytemuck::cast_slice(desc.indices), rgraph, cache, device);
        let positions = upload_buffer(bytemuck::cast_slice(desc.positions), rgraph, cache, device);
        let normals = upload_buffer(bytemuck::cast_slice(desc.normals), rgraph, cache, device);
        let uvs = upload_buffer(bytemuck::cast_slice(desc.uvs), rgraph, cache, device);

        Self {
            indices_count: desc.indices.len() as u32,
            indices,
            positions,
            normals,
            uvs,
        }
    }
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
