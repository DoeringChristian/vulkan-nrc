use indexmap::IndexMap;
use screen_13::prelude::*;
use std::sync::Arc;

use crate::instance::InstanceData;
use crate::registry::{BufferIndex, Register, Registry};

use super::{bsdf::BSDF, emitter::Emitter, instance::Instance, shape::Shape};

#[derive(Default)]
pub struct Scene {
    pub instances: Vec<Instance>,
    pub shapes: Vec<Box<dyn Shape>>,
    pub bsdfs: Vec<Box<dyn BSDF>>,
    pub emitters: Vec<Box<dyn Emitter>>,

    pub buffers: Vec<Arc<Buffer>>,
    pub images: Vec<Arc<Image>>,
    pub callables: Vec<Shader>,
    pub instance_buffer: Option<BufferIndex>,
}

impl Scene {
    pub fn upload(&mut self, device: &Arc<Device>) {
        let mut registry = Registry::new(device);

        let instance_data = self
            .instances
            .iter()
            .map(|instance| {
                let shape = &*self.shapes[instance.shape];
                let shape_buf = shape.register(&mut registry);
                let compute_surface_interaction =
                    registry.add_callable(shape.compute_surface_interaction());

                let bsdf = &*self.bsdfs[instance.bsdf];
                let bsdf_buf = bsdf.register(&mut registry);
                let bsdf_eval = registry.add_callable(bsdf.eval());
                let bsdf_sample = registry.add_callable(bsdf.sample());

                let emitter = &*self.emitters[instance.emitter];
                let emitter_buf = emitter.register(&mut registry);
                let emitter_eval = registry.add_callable(emitter.eval());
                let emitter_sample = registry.add_callable(emitter.sample());

                InstanceData {
                    to_world: instance.to_world,
                    shape_buf,
                    compute_surface_interaction,
                    bsdf_buf,
                    bsdf_eval,
                    bsdf_sample,
                    emitter_buf,
                    emitter_eval,
                    emitter_sample,
                }
            })
            .collect::<Vec<_>>();

        let instance_buffer = registry.upload_std140_slice(&instance_data);
        self.instance_buffer = Some(instance_buffer);

        let Registry {
            mut cache,
            rgraph,
            mut buffers,
            mut images,
            callables,
            ..
        } = registry;
        rgraph.resolve().submit(&mut cache, 0, 0).unwrap();

        self.callables.clear();
        self.callables.extend(callables.into_values());

        std::mem::swap(&mut self.buffers, &mut buffers);
        std::mem::swap(&mut self.images, &mut images);
    }
}
