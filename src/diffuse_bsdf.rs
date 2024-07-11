use std::sync::Arc;

use crevice::glsl::GlslStruct;
use crevice::std140::AsStd140;

use crate::bsdf::BSDF;
use crate::registry::{BufferIndex, BufferOffset, ImageIndex, Registry};
use screen_13::prelude::*;

struct DiffuseBSDF {
    pub value: Arc<Image>,
}

impl BSDF for DiffuseBSDF {
    fn eval(&self) -> &'static [u32] {
        todo!()
    }

    fn sample(&self) -> &'static [u32] {
        todo!()
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
