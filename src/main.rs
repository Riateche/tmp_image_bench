use image::{ImageFormat, RgbaImage};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn load(image_path: impl AsRef<Path>) -> RgbaImage {
    let f = BufReader::new(File::open(image_path.as_ref()).unwrap());
    image::load(f, ImageFormat::Png).unwrap().to_rgba()
}

fn main() {
    let image2 = load("2.png");

    let mut image1 = load("1.png");
    image_bench::copy_rect_by_pixel(&mut image1, &image2);
    image1.save("/tmp/3a.png").unwrap();

    let mut image1a = load("1.png");
    image_bench::copy_rect_by_line(&mut image1a, &image2);
    image1a.save("/tmp/3b.png").unwrap();

    assert_eq!(image1, image1a);
}
