use glam::*;
use screen_13::prelude::*;
use screen_13_window::{Window, WindowError};

use self::diffuse_bsdf::{DiffuseBSDF, DiffuseBSDFDesc};
use self::instance::Instance;
use self::mesh::{Mesh, MeshDesc};
use self::registry::Registry;
use self::scene::Scene;

mod area_emitter;
mod bsdf;
mod device_address;
mod diffuse_bsdf;
mod emitter;
mod instance;
mod mesh;
mod registry;
mod sbt;
mod scene;
mod shape;

fn main() -> Result<(), WindowError> {
    pretty_env_logger::init();

    let window = Window::new()?;
    let device = &window.device;

    let mut registry = Registry::new(device);

    let indices = vec![0, 1, 2];
    let positions = vec![[0., 0., 0.], [0., 1., 0.], [1., 0., 0.]];
    let normals = vec![[0., 0., 1.], [0., 0., 1.], [0., 0., 1.]];
    let uvs = vec![[0., 0.], [0., 1.], [1., 0.]];

    let mesh = Mesh::new(
        MeshDesc {
            indices: &indices,
            positions: &positions,
            normals: &normals,
            uvs: &uvs,
        },
        &mut registry,
    );

    let bsdf = DiffuseBSDF::new(
        DiffuseBSDFDesc::LinearRGB8 { r: 255, g: 0, b: 0 },
        &mut registry,
    );

    let mut scene = Scene {
        instances: vec![Instance {
            to_world: Mat4::IDENTITY,
            shape: 0,
            bsdf: 0,
            emitter: 0,
        }],
        shapes: vec![Box::new(mesh)],
        bsdfs: vec![Box::new(bsdf)],
        emitters: vec![],
        ..Default::default()
    };

    scene.upload(registry);

    window.run(|frame| {
        frame
            .render_graph
            .clear_color_image_value(frame.swapchain_image, [100u8, 149, 237]);
    })
}
