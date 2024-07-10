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
                let shape_buf = self.shapes[instance.shape].register(&mut registry);
                InstanceData {
                    to_world: instance.to_world,
                    shape_buf,
                    intersection_callable: 0,
                    compute_surface_interaction_callable: 0,
                }
            })
            .collect::<Vec<_>>();
    }
}
