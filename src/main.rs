use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let path = if args.len() == 1 {
        "walnut.jpg"
    } else {
        &args[1][..]
    };
    let img = ImageReader::open(path).unwrap().decode().unwrap();

    print!("{}", get_image(img, 10));
}

fn get_image(img: DynamicImage, scale: u32) -> String {
    let mut emoji_art = String::new();
    for y in 0..img.height() {
        for x in 0..img.width() {
            if y % (scale * 2) != 0 || x % scale != 0 {
                continue;
            }
            let pixel = img.get_pixel(x, y);
            let rgb = [pixel[0], pixel[1], pixel[2]];
            emoji_art.push(get_str_emoji(rgb));
        }
        if y % (scale * 2) == 0 {
            emoji_art.push('\n');
        }
    }
    emoji_art
}

fn get_str_emoji(rgb: [u8; 3]) -> char {
    let emoji = HashMap::from([
        ([244, 65, 51], '🟥'),
        ([255, 205, 46], '🟨'),
        ([18, 119, 211], '🟦'),
        ([172, 69, 189], '🟪'),
        ([255, 153, 0], '🟧'),
        ([184, 109, 83], '🟫'),
        ([64, 64, 64], '⬛'),
        ([225, 225, 225], '⬜'),
        ([125, 180, 64], '🟩'),
    ]);

    let square_colors = emoji.keys().collect::<Vec<_>>();
    let mut closest_rgb = square_colors[0];
    let mut diff = difference(rgb, *square_colors[0]);
    for i in 1..square_colors.len() {
        let new_diff = difference(rgb, *square_colors[i]);
        if new_diff <= diff {
            closest_rgb = square_colors[i];
            diff = new_diff;
        }
    }
    return emoji[closest_rgb];
}

fn difference(rgb1: [u8; 3], rgb2: [u8; 3]) -> u32 {
    return rgb1[0].abs_diff(rgb2[0]) as u32
        + rgb1[1].abs_diff(rgb2[1]) as u32
        + rgb1[2].abs_diff(rgb2[2]) as u32;
}
