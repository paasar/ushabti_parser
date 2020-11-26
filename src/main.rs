extern crate image;

use image::{GenericImageView};

const GREEN_SCALE: ([u8;3], [u8;3]) = ([0, 80, 0], [100, 255, 100]);

fn is_within(values: [u8;3], acceptable_range: ([u8;3], [u8;3])) -> bool {
    let [r, g, b] = values;
    let [min_r, min_g, min_b] = acceptable_range.0;
    let [max_r, max_g, max_b] = acceptable_range.1;

    return min_r < r && r < max_r &&
           min_g < g && g < max_g &&
           min_b < b && b < max_b;
}

fn main() {
    println!("Doing some image magic!");

    let img = image::open("ushabti_1_small.jpg").unwrap();

    // The dimensions method returns the images width and height.
    let dim= img.dimensions();
    let (width, height) = dim;
    println!("dimensions {:?}", dim);

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let original_pixel = img.get_pixel(x, y);
        let image::Rgba(data) = original_pixel;
        if is_within([data[0], data[1], data[2]], GREEN_SCALE) {
            *pixel = image::Rgb([data[0], data[1], data[2]])
        } else {
            *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
        }
    }

    imgbuf.save("output.png").unwrap();
}
