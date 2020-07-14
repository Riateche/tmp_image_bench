use criterion::{criterion_group, criterion_main, Criterion};
use image::RgbaImage;
use image_bench::{copy_rect_by_line, copy_rect_by_pixel};

fn images() -> (RgbaImage, RgbaImage) {
    (RgbaImage::new(1000, 1000), RgbaImage::new(500, 500))
}

pub fn by_pixel(c: &mut Criterion) {
    let (mut image1, image2) = images();
    c.bench_function("by_pixel", move |b| {
        b.iter(|| {
            copy_rect_by_pixel(&mut image1, &image2);
        })
    });
}

pub fn by_line(c: &mut Criterion) {
    let (mut image1, image2) = images();
    c.bench_function("by_line", move |b| {
        b.iter(|| {
            copy_rect_by_line(&mut image1, &image2);
        })
    });
}

criterion_group!(benches, by_pixel, by_line);
criterion_main!(benches);
