extern crate pentaxctl;

use pentaxctl::Camera;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let camera = Camera::new("192.168.0.1".to_string());

    // Take a one second exposure when in bulb mode.
    camera.shutter_press()
        .expect("could not press shutter");
    sleep(Duration::from_millis(1000));
    camera.shutter_release()
        .expect("could not release shutter");
}
