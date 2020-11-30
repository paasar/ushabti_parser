extern crate image;

use image::{GenericImageView};
use std::cmp::{max, min};

//TODO Other ushabti colors
const GREEN_SCALE: ([u8;3], [u8;3]) = ([0, 80, 0], [120, 255, 120]);
const PIXEL_SURROUND_RANGE: u32 = 10;
const PIXEL_SURROUND_AREA: u32 = PIXEL_SURROUND_RANGE * PIXEL_SURROUND_RANGE;

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

    let dim= img.dimensions();
    let (width, height) = dim;
    println!("Dimensions {:?}, ColorType {:?}", dim, img.color());

    let mut result_img_buf = image::ImageBuffer::new(width, height);

    let mut color_array: Vec<Vec<[u8; 3]>> = vec![vec![[0,0,0]; height as usize]; width as usize];

    // TODO I'd like to enumerate the actual image
    for (x, y, _) in result_img_buf.enumerate_pixels() {
        let image::Rgba(data) = img.get_pixel(x, y);
        color_array[x as usize][y as usize] = [data[0], data[1], data[2]]
    }

    for (x, y, pixel) in result_img_buf.enumerate_pixels_mut() {
        // This is a bit unsafe, but I expect to handle smaller images than width/height of i32::MAX.
        let min_x = max(x as i32 - PIXEL_SURROUND_RANGE as i32, 0) as u32;
        let max_x = min(x + PIXEL_SURROUND_RANGE, width);
        let min_y = max(y as i32 - PIXEL_SURROUND_RANGE as i32, 0) as u32;
        let max_y = min(y + PIXEL_SURROUND_RANGE, height);

        // TODO Slow?
        let mut surrounding_pixels = Vec::new();
        for sx in min_x..max_x {
            for sy in min_y..max_y {
                surrounding_pixels.push(color_array[sx as usize][sy as usize]);
            }
        }

        let mut within = 0;
        let original_rgb = color_array[x as usize][y as usize];
        for px in surrounding_pixels {
            if is_within(px, GREEN_SCALE) {
                within += 1;
            }
        }

        if within as f64 / PIXEL_SURROUND_AREA as f64 > 0.2 {
            *pixel = image::Rgb(original_rgb)
        } else {
            *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
        }
    }

    result_img_buf.save("output.png").unwrap();
}
