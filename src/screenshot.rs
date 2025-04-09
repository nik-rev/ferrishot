use image::{ImageBuffer, Rgba};

pub fn gete() -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    image::ImageReader::open("../monitor.png")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba8()
}

pub fn get() -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let mouse_position::mouse_position::Mouse::Position { x, y } =
        mouse_position::mouse_position::Mouse::get_mouse_position()
    else {
        panic!("Could not get mouse position")
    };

    let monitor = xcap::Monitor::from_point(x, y).expect("Could not get monitor");

    monitor.capture_image().unwrap()
}
