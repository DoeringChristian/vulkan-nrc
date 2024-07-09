use screen_13::prelude::*;
use screen_13_window::{Window, WindowError};

fn main() -> Result<(), WindowError> {
    pretty_env_logger::init();
    Window::new()?.run(|frame| {
        frame
            .render_graph
            .clear_color_image_value(frame.swapchain_image, [100u8, 149, 237]);
    })
}
