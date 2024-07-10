use screen_13::prelude::*;
use screen_13_window::{Window, WindowError};

mod bsdf;
mod device_address;
mod instance;
mod mesh;
mod registry;
mod sbt;
mod scene;
mod shape;

fn main() -> Result<(), WindowError> {
    let v = vec![];
    let w = crevice::std140::Writer::new(v);

    pretty_env_logger::init();
    Window::new()?.run(|frame| {
        frame
            .render_graph
            .clear_color_image_value(frame.swapchain_image, [100u8, 149, 237]);
    })
}
