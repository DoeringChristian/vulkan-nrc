use indexmap::IndexMap;
use screen_13::prelude::*;
use std::sync::Arc;

use crate::instance::InstanceData;
use crate::registry::{Register, Registry};

use super::{bsdf::BSDF, instance::Instance, shape::Shape};

pub struct Scene {
    pub instances: Vec<Instance>,
    pub shapes: Vec<Box<dyn Shape>>,
    pub bsdfs: Vec<Box<dyn BSDF>>,

    pub buffers: Vec<Arc<Buffer>>,
    pub images: Vec<Arc<Image>>,
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

                InstanceData {
                    to_world: instance.to_world,
                    shape_buf,
                    compute_surface_interaction,
                }
            })
            .collect::<Vec<_>>();
    }
}
