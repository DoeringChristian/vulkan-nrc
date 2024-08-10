use crevice::glsl::GlslStruct;
use crevice::std140::AsStd140;
use screen_13::prelude::*;
use std::sync::Arc;

use crate::emitter::Emitter;
use crate::registry::{BufferIndex, BufferOffset, ImageIndex, Registry};

pub enum AreaEmitterDesc {
    LinearRGB8 { r: u8, g: u8, b: u8 },
}

struct AreaEmitter {
    pub value: Arc<Image>,
}

impl AreaEmitter {
    pub fn new(desc: AreaEmitterDesc, registry: &mut Registry) -> Self {
        match desc {
            AreaEmitterDesc::LinearRGB8 { r, g, b } => Self {
                value: registry.upload_linear_rgb8(&[r, g, b], 1, 1),
            },
        }
    }
}

impl Emitter for AreaEmitter {
    fn eval(&self) -> &'static [u32] {
        todo!()
    }

    fn sample(&self) -> &'static [u32] {
        todo!()
    }

    fn register(&self, registry: &mut Registry) -> (BufferIndex, BufferOffset) {
        let data = AreaEmitterData {
            value: registry.add_image(&self.value),
        };
        registry.upload_std140(data)
    }
}

#[derive(AsStd140, GlslStruct, Debug, Clone, Copy)]
pub struct AreaEmitterData {
    pub value: ImageIndex,
}
