mod args;
use std::{io::BufReader, fs::File};

use args::Args;
use image::{ io::Reader, DynamicImage, ImageFormat };

fn main() {
    let args = Args::new();
    println!("{:?}",args);

    let (image_1, image_1_format) = find_image_from_path(args.image_1);

    let (image_2, image_2_format) = find_image_from_path(args.image_2);
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}
