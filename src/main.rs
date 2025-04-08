#![cfg_attr(doc, doc = include_str!("../README.md"))]

use std::time::Instant;
use xcap::Monitor;

fn normalized(filename: String) -> String {
    filename.replace(['|', '\\', ':', '/'], "")
}

fn main() {
    let start = Instant::now();
    let monitors = Monitor::all().unwrap();

    for monitor in monitors {
        let image = monitor.capture_image().unwrap();

        image
            .save(format!(
                "monitor-{}.png",
                normalized(monitor.name().unwrap())
            ))
            .unwrap();
    }

    println!("took: {:?}", start.elapsed());
}
