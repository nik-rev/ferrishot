pub fn screenshot() {
    let mouse_position::mouse_position::Mouse::Position { x, y } =
        mouse_position::mouse_position::Mouse::get_mouse_position()
    else {
        panic!("Could not get mouse position")
    };

    let monitor = xcap::Monitor::from_point(x, y).expect("Could not get monitor");

    let image = monitor.capture_image().unwrap();

    image
        .save(format!("monitor-{}.png", monitor.name().unwrap()))
        .unwrap();
}
