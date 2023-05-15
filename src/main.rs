/*
sudo apt-get install -y libv4l-dev
sudo apt-get install v4l-utils
sudo apt-get install gettext
sudo apt-get install v4l2loopback-dkms
sudo apt install -y libclang-dev
 */

use rpcamrs::rpcamrs::*;
use std::fs;
use std::io::Write;

fn main() {
    use rscam::{Camera, Config};

    let mut camera = Camera::new("/dev/video0").unwrap();

    camera
        .start(&Config {
            ..Default::default()
        })
        .unwrap();

    for i in 0..10 {
        let frame = camera.capture().unwrap();
        let mut file = fs::File::create(&format!("frame-{}.jpg", i)).unwrap();
        file.write_all(&frame[..]).unwrap();
    }
}
