extern crate image;

use std::cmp::{max, min};

use image::{GenericImageView, ImageBuffer, Rgb};

const RED: [u8;3] = [255, 0, 0];
const GREEN: [u8;3] = [0, 255, 0];
const WHITE: [u8;3] = [255, 255, 255];
const BLACK: [u8;3] = [0, 0, 0];
//TODO Other ushabti colors
const GREEN_SCALE: ([u8;3], [u8;3]) = ([0, 80, 0], [120, 255, 120]);
const SEEK_STEP: u32 = 60;
const DEFINITION_STEP: u32 = 10;
const PIXEL_SURROUND_RANGE: u32 = 10;
const PIXEL_SURROUND_AREA: u32 = PIXEL_SURROUND_RANGE * PIXEL_SURROUND_RANGE;

fn main() {
    println!("Doing some image magic!");

    // let img = image::open("ushabti_4_small.png").unwrap();
    let img = image::open("images/ushabti_1_small.jpg").unwrap();

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
        if x % SEEK_STEP == 0 && y % SEEK_STEP == 0 && !point_in_any_of(x, y, &found_ushabtis) {

            if is_ushabti_pixel(x, y, width, height, &color_array) {
                resolve_and_add_ushabti(x, y, width, height, &color_array, &mut found_ushabtis);
                *pixel = image::Rgb(RED);
            } else {
                *pixel = image::Rgb(GREEN);
            }

        } else {
            let original_rgb = color_array[x as usize][y as usize];
            *pixel = image::Rgb(original_rgb)
        }
    }

    draw_bounding_box_around_ushabtis(&found_ushabtis, &mut result_img_buf);
    try_to_recognize_each_symbol(&found_ushabtis, color_array);

    result_img_buf.save("output.png").unwrap();
}

fn is_groove_dark_pixel([r, g, b]: [u8; 3]) -> bool {
    return ((r as f32 + g as f32 + b as f32) / 3 as f32) < 80 as f32;
}

fn symbol_bottom_adjustment(y_top: u32, y_bottom: u32) -> u32 {
    if y_bottom - y_top > 180 {
        // A big ushabti has the symbol a bit higher
        return 14;
    } else {
        return 0;
    }
}

fn try_to_recognize_each_symbol(found_ushabtis: &Vec<[u32; 4]>, color_array: Vec<Vec<[u8; 3]>>) {
    let symbol_ankh = image::open("symbol_ankh.png").unwrap();
    let symbol_bat = image::open("symbol_bat.png").unwrap();
    let symbol_eye = image::open("symbol_eye.png").unwrap();
    let symbol_snake = image::open("symbol_snake.png").unwrap();
    let symbol_vortex = image::open("symbol_vortex.png").unwrap();

    let mut count = 1;
    for [x1, y1, x2, y2] in found_ushabtis.to_vec() {
        let ushabi_edge_to_symbol_edge = 30;
        let symbol_left = x1 + ushabi_edge_to_symbol_edge;
        let symbol_right = x2 - ushabi_edge_to_symbol_edge;
        let symbol_height = 30;
        let from_bottom_to_symbol_bottom = 20;
        let symbol_bottom = y2 - from_bottom_to_symbol_bottom - symbol_bottom_adjustment(y1, y2);
        let symbol_top = symbol_bottom - symbol_height;

        println!("Found a symbol at {:?}, {:?}, {:?}, {:?}", symbol_left, symbol_top, symbol_right, symbol_bottom);

        let mut symbol_image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(symbol_right - symbol_left, symbol_height);
        // copy pixels
        for (x, y, pix) in symbol_image_buf.enumerate_pixels_mut() {
            let color = color_array[(symbol_left + x) as usize][(symbol_top + y) as usize];
            if is_groove_dark_pixel(color) {
                *pix = image::Rgb(WHITE);
            } else {
                *pix = image::Rgb(BLACK);
            }
        }

        let total_pixels = symbol_image_buf.width() * symbol_image_buf.height();
        let mut ankh_match = 0;
        let mut bat_match = 0;
        let mut eye_match = 0;
        let mut snake_match = 0;
        let mut vortex_match = 0;

        // TODO This can panic if the symbol dimensions are different for any reason
        for (x, y, pix) in symbol_image_buf.enumerate_pixels() {
            let image::Rgba([ankh_r, _, _, _]) = symbol_ankh.get_pixel(x, y);
            let image::Rgba([bat_r, _, _, _]) = symbol_bat.get_pixel(x, y);
            let image::Rgba([eye_r, _, _, _]) = symbol_eye.get_pixel(x, y);
            let image::Rgba([snake_r, _, _, _]) = symbol_snake.get_pixel(x, y);
            let image::Rgba([vortex_r, _, _, _]) = symbol_vortex.get_pixel(x, y);
            let image::Rgb([symbol_r, _, _]) = pix;

            if ankh_r == *symbol_r {
                ankh_match += 1;
            }

            if bat_r == *symbol_r {
                bat_match += 1;
            }

            if eye_r == *symbol_r {
                eye_match += 1;
            }

            if snake_r == *symbol_r {
                snake_match += 1;
            }

            if vortex_r == *symbol_r {
                vortex_match += 1;
            }
        }

        let ankh_similarity = ((ankh_match as f32 / total_pixels as f32) * 100 as f32, "Ankh");
        // println!("Ankh similarity {:?} %.", ankh_similarity.0);
        let bat_similarity = ((bat_match as f32 / total_pixels as f32) * 100 as f32, "Bat");
        // println!("Bat similarity {:?} %.", bat_similarity.0);
        let eye_similarity = ((eye_match as f32 / total_pixels as f32) * 100 as f32, "Eye");
        // println!("Eye similarity {:?} %.", eye_similarity.0);
        let snake_similarity = ((snake_match as f32 / total_pixels as f32) * 100 as f32, "Snake");
        // println!("Snake similarity {:?} %.", snake_similarity.0);
        let vortex_similarity = ((vortex_match as f32 / total_pixels as f32) * 100 as f32, "Vortex");
        // println!("Vortex similarity {:?} %.", vortex_similarity.0);

        let mut similarities = [ankh_similarity, bat_similarity, eye_similarity, snake_similarity, vortex_similarity];
        similarities.sort_by(|(sim1, _), (sim2, _) | sim2.partial_cmp(sim1).unwrap());

        println!("Symbol (probably) is: {:?}", similarities[0].1);

        // separate name for each symbol
        symbol_image_buf.save(format!("output_symbol_{}.png", count)).unwrap();
        count += 1;
    }
}

fn point_in_any_of(x: u32, y: u32, area_vec: &Vec<[u32;4]>) -> bool {
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

fn resolve_and_add_ushabti(x: u32, y: u32, width: u32, height: u32, color_array: &Vec<Vec<[u8; 3]>>, found_ushabtis: &mut Vec<[u32; 4]>) {
    match resolve_shape(x, y, width, height, &color_array) {
        Some(found_shape) => {
            match overlaps_with_any(found_shape, &found_ushabtis) {
                Some((position, area_of_existing)) => {
                    if area_of_existing < area(found_shape) {
                        found_ushabtis.remove(position);
                        found_ushabtis.push(found_shape);
                        println!("Bigger than previous ushabti at {:?}", found_shape);
                    }
                },
                None => {
                    found_ushabtis.push(found_shape);
                    println!("Ushabti at {:?}", found_shape);
                }
            }
        },
        None => ()
    }
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
    cur_x = (bottom_right_x + top_left_x) / 2;
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
    cur_y = (bottom_right_y + top_left_y) / 2;
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
        bottom_right_y - top_left_y > 140;
}

fn overlaps_with_any(rect: [u32; 4], found_ushabtis: &Vec<[u32; 4]>) -> Option<(usize, u32)> {
    for (position, ushabti) in found_ushabtis.iter().enumerate() {
        if overlaps(rect, *ushabti) {
            return Some((position, area(*ushabti)));
        }
    }

    return None;
}

fn overlaps(rect_a: [u32; 4], rect_b: [u32; 4]) -> bool {
    let [a_x1, a_y1, a_x2, a_y2] = rect_a;
    let [b_x1, b_y1, b_x2, b_y2] = rect_b;

    return !(a_x2 < b_x1 || a_x1 > b_x2 || a_y2 < b_y1 || a_y1 > b_y2)
}

fn area(rect: [u32; 4]) -> u32 {
    let [x1, y1, x2, y2] = rect;

    // Theoretical possibility to go over u32
    return (x2 - x1) * (y2 - y1);
}

fn draw_bounding_box_around_ushabtis(found_ushabtis: &Vec<[u32; 4]>, result_img_buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for [x1, y1, x2, y2] in found_ushabtis.to_vec() {
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
