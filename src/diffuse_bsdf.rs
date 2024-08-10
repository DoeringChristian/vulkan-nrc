use std::path::Path;
use std::sync::Arc;

use crevice::glsl::GlslStruct;
use crevice::std140::AsStd140;
use inline_spirv::include_spirv;

use crate::bsdf::BSDF;
use crate::registry::{BufferIndex, BufferOffset, ImageIndex, Registry};
use screen_13::prelude::*;

pub enum DiffuseBSDFDesc {
    LinearRGB8 { r: u8, g: u8, b: u8 },
}

pub struct DiffuseBSDF {
    pub value: Arc<Image>,
}

impl DiffuseBSDF {
    pub fn new(desc: DiffuseBSDFDesc, registry: &mut Registry) -> Self {
        match desc {
            DiffuseBSDFDesc::LinearRGB8 { r, g, b } => Self {
                value: registry.upload_linear_rgb8(&[r, g, b], 1, 1),
            },
        }
    }
}

impl BSDF for DiffuseBSDF {
    fn eval(&self) -> &'static [u32] {
        include_spirv!("src/shaders/bsdf/diffuse_bsdf/eval.glsl", rcall, vulkan1_2,  I "src/shaders")
    }

    fn sample(&self) -> &'static [u32] {
        include_spirv!("src/shaders/bsdf/diffuse_bsdf/sample.glsl", rcall, vulkan1_2,  I "src/shaders")
    }

    fn register(&self, registry: &mut Registry) -> (BufferIndex, BufferOffset) {
        let data = DiffuseBSDFData {
            value: registry.add_image(&self.value),
        };
        registry.upload_std140(data)
    }
}

#[derive(AsStd140, GlslStruct, Debug, Clone, Copy)]
pub struct DiffuseBSDFData {
    pub value: ImageIndex,
}
