use std::sync::Arc;

use crevice::glsl::GlslStruct;
use crevice::std140::AsStd140;

use crate::bsdf::BSDF;
use crate::registry::{ImageIndex, Register};
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
}

impl Register for DiffuseBSDF {
    fn register(&self, registry: &mut crate::registry::Registry) -> crate::registry::BufferIndex {
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
