extern crate pentaxctl;

use pentaxctl::Camera;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let camera = Camera::new("192.168.0.1".to_string());

    println!("Setting ISO to 1600");
    camera.set_iso(1600).expect("could not set ISO");

    // Take an exposure when in bulb mode.
    println!("Taking photo...");
    camera
        .shutter_press()
        .expect("could not press shutter (are you in Bulb?)");

    sleep(Duration::from_millis(250));

    camera.shutter_release().expect("could not release shutter");

    println!("Downloading image...");
    let image = camera
        .get_latest_capture()
        .expect("could not download image from camera");

    println!("Image downloaded. Size: {} bytes.", image.len());

    let mut wtr = BufWriter::new(
        File::create("image.dng").expect("could not create file"),
    );

    wtr.write_all(&image).expect("could not save image");

    println!("Image saved.");
}
