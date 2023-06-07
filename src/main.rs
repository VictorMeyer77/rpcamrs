/*
sudo apt-get install -y libv4l-dev 1
build realese
 */

use image::imageops::FilterType;
use image::io::{Reader as ImageReader, Reader};
use image::{save_buffer, ColorType, DynamicImage, EncodableLayout, ImageBuffer, Pixel, RgbaImage};
use std::io::{Cursor, Write};
use std::path::Path;
use std::time::{Duration, SystemTime};
use std::{fs, thread};
use tensorflow::{Graph, SavedModelBundle, SessionOptions, SessionRunArgs, Tensor};

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

        let img3 = img2.resize_exact(64, 64, FilterType::Triangle);

        //let img = ImageReader::open("frame-0.jpg").unwrap().decode().unwrap();
        println!("{:?}", (img3.width(), img3.height()));
        println!("{}", img3.as_bytes().len());

        save_buffer(
            &Path::new("image.jpg"),
            img3.as_bytes(),
            img3.width(),
            img3.height(),
            ColorType::Rgb8,
        )
        .unwrap();

        let testa: Vec<f32> = img3
            .as_bytes()
            .iter()
            .map(|&x| (x as f32) / 255.0)
            .collect();

        println!("{:?}", testa.as_slice().len());

        predict(testa.as_slice())
    }
}

fn predict(im: &[f32]) {
    // In this file test_in_input is being used while in the python script,
    // that generates the saved model from Keras model it has a name "test_in".
    // For multiple inputs _input is not being appended to signature input parameter name.
    let signature_input_parameter_name = "test_in_input";
    let signature_output_parameter_name = "test_out";

    // Initialize save_dir, input tensor, and an empty graph
    let save_dir = "/home/victor/Entreprise/Developpement/mr-train-model/alpyne/input/test_model";
    let tensor: Tensor<f32> = Tensor::new(&[1, 64, 64, 3])
        .with_values(im)
        .expect("Can't create tensor");
    let mut graph = Graph::new();

    // Load saved model bundle (session state + meta_graph data)
    let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, save_dir)
        .expect("Can't load saved model");

    // Get the session from the loaded model bundle
    let session = &bundle.session;

    // Get signature metadata from the model bundle
    let signature = bundle
        .meta_graph_def()
        .get_signature("serving_default")
        .unwrap();

    // Get input/output info
    let input_info = signature.get_input(signature_input_parameter_name).unwrap();
    let output_info = signature
        .get_output(signature_output_parameter_name)
        .unwrap();

    // Get input/output ops from graph
    let input_op = graph
        .operation_by_name_required(&input_info.name().name)
        .unwrap();
    let output_op = graph
        .operation_by_name_required(&output_info.name().name)
        .unwrap();

    // Manages inputs and outputs for the execution of the graph
    let mut args = SessionRunArgs::new();
    args.add_feed(&input_op, 0, &tensor); // Add any inputs

    let out = args.request_fetch(&output_op, 0); // Request outputs

    // Run model
    session
        .run(&mut args) // Pass to session to run
        .expect("Error occurred during calculations");

    // Fetch outputs after graph execution
    let out_res: Tensor<f32> = args.fetch(out).unwrap();

    println!("Results: {:?}", out_res);
}
