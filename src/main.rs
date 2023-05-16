/*
sudo apt-get install -y libv4l-dev 1
build realese
 */

use image::io::{Reader as ImageReader, Reader};
use image::{save_buffer, ColorType, EncodableLayout, ImageBuffer, Pixel, RgbaImage};
use std::io::{Cursor, Write};
use std::path::Path;
use std::time::{Duration, SystemTime};
use std::{fs, thread};

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
    thread::sleep(Duration::from_millis(1000));

    for i in 0..1 {
        let frame = camera.capture().unwrap();
        //let mut file = fs::File::create(&format!("frame-{}.jpg", i)).unwrap();
        //file.write_all(&frame[..]).unwrap();

        //println!("{:?}", frame);

        let img2 = Reader::new(Cursor::new(&frame[..]))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        //let img = ImageReader::open("frame-0.jpg").unwrap().decode().unwrap();
        println!("{:?}", (img2.width(), img2.height()));
        println!("{}", img2.as_bytes().len());

        save_buffer(
            &Path::new("image.jpg"),
            img2.as_bytes(),
            img2.width(),
            img2.height(),
            ColorType::Rgb8,
        )
        .unwrap();
    }
}
