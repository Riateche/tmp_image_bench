use image::RgbaImage;

fn copy_line(
    src: &RgbaImage,
    src_x: u32,
    src_y: u32,
    dst: &mut RgbaImage,
    dst_x: u32,
    dst_y: u32,
    width: u32,
) {
    let src = src.as_flat_samples();
    let src_index = src.index(0, src_x, src_y).unwrap();
    assert!(src_x + width <= src.extents().1 as u32);
    let count = width as usize * src.extents().0;

    let mut dst = dst.as_flat_samples_mut();
    let dst_index = dst.index(0, dst_x, dst_y).unwrap();
    assert!(dst_x + width <= dst.extents().1 as u32);

    let src_slice = &src.as_slice()[src_index..src_index + count];
    let dst_slice = &mut dst.as_mut_slice()[dst_index..dst_index + count];
    dst_slice.copy_from_slice(src_slice);
}

const OFFSET: u32 = 300;
const SIZE: u32 = 500;

pub fn copy_rect_by_pixel(image1: &mut RgbaImage, image2: &RgbaImage) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            image1.put_pixel(OFFSET + x, OFFSET + y, *image2.get_pixel(x, y));
        }
    }
}
pub fn copy_rect_by_line(image1: &mut RgbaImage, image2: &RgbaImage) {
    for y in 0..SIZE {
        copy_line(image2, 0, y, image1, OFFSET, y + OFFSET, SIZE);
    }
}
