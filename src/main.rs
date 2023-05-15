/*
sudo apt-get install -y libv4l-dev
 */

use rpcamrs::rpcamrs::*;
use std::fs;
use std::io::Write;

fn main() {
    use rscam::{Camera, Config};

    let mut camera = Camera::new("/dev/video0").unwrap();

    camera
        .start(&Config {
            interval: (1, 30), // 30 fps.
            resolution: (1280, 720),
            format: b"MJPG",
            ..Default::default()
        })
        .unwrap();

    for i in 0..10 {
        let frame = camera.capture().unwrap();
        let mut file = fs::File::create(&format!("frame-{}.jpg", i)).unwrap();
        file.write_all(&frame[..]).unwrap();
    }
}
