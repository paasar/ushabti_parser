extern crate image;

use std::cmp::{max, min};

use image::{GenericImageView, ImageBuffer, Rgb};

const RED: [u8;3] = [255, 0, 0];
const GREEN: [u8;3] = [0, 255, 0];
const WHITE: [u8;3] = [255, 255, 255];
//TODO Other ushabti colors
const GREEN_SCALE: ([u8;3], [u8;3]) = ([0, 80, 0], [120, 255, 120]);
const SEEK_STEP: u32 = 60;
const DEFINITION_STEP: u32 = 10;
const PIXEL_SURROUND_RANGE: u32 = 10;
const PIXEL_SURROUND_AREA: u32 = PIXEL_SURROUND_RANGE * PIXEL_SURROUND_RANGE;

fn main() {
    println!("Doing some image magic!");

    let img = image::open("ushabti_1.jpeg").unwrap();
    // let img = image::open("ushabti_1_small.jpg").unwrap();
    // let img = image::open("ushabti_1_tiny.png").unwrap();

    let dim= img.dimensions();
    let (width, height) = dim;
    println!("Dimensions {:?}, ColorType {:?}", dim, img.color());

    let mut result_img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(width, height);

    let mut color_array: Vec<Vec<[u8; 3]>> = vec![vec![[0,0,0]; height as usize]; width as usize];
    // TODO "A non-constant value was used in a constant expression"
    // let mut color_array = [[[0,0,0]; height as usize]; width as usize];

    // TODO I'd like to enumerate the actual image
    for (x, y, _) in result_img_buf.enumerate_pixels() {
        let image::Rgba(data) = img.get_pixel(x, y);
        color_array[x as usize][y as usize] = [data[0], data[1], data[2]]
    }

    let mut found_ushabtis: Vec<[u32; 4]> = Vec::new();

    for (x, y, pixel) in result_img_buf.enumerate_pixels_mut() {
        if x % SEEK_STEP == 0 && y % SEEK_STEP == 0 && !point_in(x, y, &found_ushabtis) {

            if is_ushabti_pixel(x, y, width, height, &color_array) {
                match resolve_shape(x, y, width, height, &color_array) {
                    Some(found_shape) => {
                        found_ushabtis.push(found_shape);
                        println!("Ushabti at {:?}", found_shape);
                    },
                    None => ()
                }
                *pixel = image::Rgb(RED);
            } else {
                *pixel = image::Rgb(GREEN);
            }

        } else {
            let original_rgb = color_array[x as usize][y as usize];
            *pixel = image::Rgb(original_rgb)
        }
    }

    draw_bounding_box_around_ushabtis(&mut result_img_buf, found_ushabtis);

    result_img_buf.save("output.png").unwrap();
}

fn point_in(x: u32, y: u32, area_vec: &Vec<[u32;4]>) -> bool {
    let areas = area_vec.to_vec();
    for [x1, y1, x2, y2] in areas {
        if x1 <= x && x <= x2 &&
            y1 <= y && y <= y2 {
            return true;
        }
    }
    return false;
}

fn is_ushabti_pixel(x: u32, y: u32, width: u32, height: u32, color_array: &Vec<Vec<[u8; 3]>>) -> bool {
    let min_x = max(x as i32 - PIXEL_SURROUND_RANGE as i32, 0) as u32;
    let max_x = min(x + PIXEL_SURROUND_RANGE, width);
    let min_y = max(y as i32 - PIXEL_SURROUND_RANGE as i32, 0) as u32;
    let max_y = min(y + PIXEL_SURROUND_RANGE, height);

    let mut surrounding_pixels = Vec::new();
    for sx in min_x..max_x {
        for sy in min_y..max_y {
            surrounding_pixels.push(color_array[sx as usize][sy as usize]);
        }
    }

    let mut num_of_surrounding_in_color_range = 0;
    for px in surrounding_pixels {
        if is_in_color_range(px, GREEN_SCALE) {
            num_of_surrounding_in_color_range += 1;
        }
    }

    return num_of_surrounding_in_color_range as f64 / PIXEL_SURROUND_AREA as f64 > 0.4;
}

fn is_in_color_range(values: [u8;3], acceptable_range: ([u8;3], [u8;3])) -> bool {
    let [r, g, b] = values;
    let [min_r, min_g, min_b] = acceptable_range.0;
    let [max_r, max_g, max_b] = acceptable_range.1;

    return min_r < r && r < max_r &&
        min_g < g && g < max_g &&
        min_b < b && b < max_b;
}

fn resolve_shape(start_x: u32, start_y: u32, width: u32, height: u32, color_array: &Vec<Vec<[u8; 3]>>) -> Option<[u32; 4]> {
    let mut top_left_x: u32 = start_x;
    let mut top_left_y: u32 = start_y;
    let mut bottom_right_x: u32 = start_x;
    let mut bottom_right_y: u32 = start_y;

    let mut cur_x: u32 = start_x;
    let mut cur_y: u32 = start_y;

    // Find right edge
    while (cur_x < width && is_ushabti_pixel(cur_x, cur_y, width, height, color_array)) ||
        (cur_x + DEFINITION_STEP < width && is_ushabti_pixel(cur_x + DEFINITION_STEP, cur_y, width, height, color_array)) {
        bottom_right_x = cur_x;
        cur_x += DEFINITION_STEP;
    }

    // Move to middle x and find min and max y.
    let half_x = (bottom_right_x - top_left_x) / 2;
    cur_x = top_left_x + half_x;
    while (cur_y > 0 && is_ushabti_pixel(cur_x, cur_y, width, height, color_array)) ||
        (cur_y > 0 + DEFINITION_STEP && is_ushabti_pixel(cur_x, cur_y - DEFINITION_STEP, width, height, color_array)) {
        top_left_y = cur_y;
        cur_y -= DEFINITION_STEP;

    }

    cur_y = start_y;
    while (cur_y < height && is_ushabti_pixel(cur_x, cur_y, width, height, color_array)) ||
        (cur_y + DEFINITION_STEP < height && is_ushabti_pixel(cur_x, cur_y + DEFINITION_STEP, width, height, color_array)) {
        bottom_right_y = cur_y;
        cur_y += DEFINITION_STEP;
    }

    // Move to middle y and find min and max x.
    let half_y = (bottom_right_y - top_left_y) / 2;
    cur_y = top_left_y + half_y;
    cur_x = start_x;
    while (cur_x > 0 && is_ushabti_pixel(cur_x, cur_y, width, height, color_array)) ||
        (cur_x > 0 + DEFINITION_STEP && is_ushabti_pixel(cur_x - DEFINITION_STEP, cur_y, width, height, color_array)) {
        top_left_x = cur_x;
        cur_x -= DEFINITION_STEP;

    }

    cur_x = start_x;
    while (cur_x < width && is_ushabti_pixel(cur_x, cur_y, width, height, color_array)) ||
        (cur_y + DEFINITION_STEP < width && is_ushabti_pixel(cur_x + DEFINITION_STEP, cur_y, width, height, color_array)) {
        bottom_right_x = cur_x;
        cur_x += DEFINITION_STEP;
    }

    if is_big_enough(top_left_x, top_left_y, bottom_right_x, bottom_right_y) {
        return Some([top_left_x, top_left_y, bottom_right_x, bottom_right_y]);
    } else {
        None
    }
}

fn is_big_enough(top_left_x: u32, top_left_y: u32, bottom_right_x: u32, bottom_right_y: u32) -> bool {
    return bottom_right_x - top_left_x > 80 &&
        bottom_right_y - top_left_y > 120;
}

fn draw_bounding_box_around_ushabtis(result_img_buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, found_ushabtis: Vec<[u32; 4]>) {
    for [x1, y1, x2, y2] in found_ushabtis {
        for xd in x1..x2 {
            let pix = result_img_buf.get_pixel_mut(xd, y1);
            *pix = image::Rgb(WHITE);
            let pix = result_img_buf.get_pixel_mut(xd, y2);
            *pix = image::Rgb(WHITE);
        }
        for yd in y1..y2 {
            let pix = result_img_buf.get_pixel_mut(x1, yd);
            *pix = image::Rgb(WHITE);
            let pix = result_img_buf.get_pixel_mut(x2, yd);
            *pix = image::Rgb(WHITE);
        }
    }
}
