use std::iter::zip;

use image::{DynamicImage, GenericImageView, Pixel, Rgb};
use rsautogui::screen::{self, on_screen, Screen};

/// Finds the target image in the primary screen. Returning a tuple of the coordinates of the top left corner of the target image.
/// - **target** - The target image that we are looking for in the source image.
/// - **looseness** - The percentage of difference allowed between the two pixels based on 255. 0 being no difference and 1 being a perfect match
pub fn find_on_screen(target: DynamicImage, looseness: f32) -> Option<(u32, u32)> {
    let (w, h) = screen::size();

    let mut screen: DynamicImage = screenshot(0, 0, w, h);

    find_target(&mut screen, &target, looseness)
}

/// Finds the target image in the source image. Returning a tuple of the coordinates of the top left corner of the target image.
/// - **source** - The source image to search for the target image.
/// - **target** - The target image that we are looking for in the source image.
/// - **looseness** - The percentage of difference allowed between the two pixels based on 255. 0 being no difference and 1 being a perfect match
pub fn find_target(
    source: &mut DynamicImage,
    target: &DynamicImage,
    looseness: f32,
) -> Option<(u32, u32)> {
    let (sw, sh) = source.dimensions();
    let (tw, th) = target.dimensions();

    let target_px = target.get_pixel(0, 0).to_rgb();

    for y in 0..sh - th {
        for x in 0..sw - tw {
            let current_px = source.get_pixel(x, y).to_rgb();

            if is_pixel_match(current_px, target_px, looseness) {
                let crop = source.crop(x, y, tw, th);
                if is_image_match(&crop, target, looseness) {
                    return Some((x, y));
                }
            }
        }
    }

    None
}


/// Finds a button on a screen using fixtures. Button is from https://codepen.io/lfdumas/pen/vOqarr.
#[cfg(test)]
mod find_target {
    use super::find_target;

    #[test]
    fn finds_button_at_0_looseness() {
        let mut source = image::open("src/__fixtures__/screen-2k.png").unwrap();
        let target = image::open("src/__fixtures__/btn.png").unwrap();

        let is_found = find_target(&mut source, &target, 0.00);
        assert_eq!(is_found, Some((1180, 934)));
    }
}

// Rgba([234, 30, 99, 255])
// x: 2000, y: 950

/// Checks if two images are similar enough to be considered a match.
/// - **img1** - The first image to compare against the second one.
/// - **img2** - The second image to compare against the first one.
/// - **looseness** - The percentage of difference allowed between the two pixels based on 255. 0 being no difference and 1 being a perfect match
pub fn is_image_match(img1: &DynamicImage, img2: &DynamicImage, looseness: f32) -> bool {
    let (w1, h1) = img1.dimensions();

    for y in 0..h1 {
        for x in 0..w1 {
            let px1 = img1.get_pixel(x, y).to_rgb();
            let px2 = img2.get_pixel(x, y).to_rgb();
            if !is_pixel_match(px1, px2, looseness) {
                return false;
            }
        }
    }

    true
}

/// Checks if two pixels are similar enough to be considered a match
/// - **px1** - The first pixel to compare
/// - **px2** - The second pixel to compare
/// - **looseness** - The percentage of difference allowed between the two pixels based on 255. 0 being no difference and 1 being a perfect match
pub fn is_pixel_match(px1: Rgb<u8>, px2: Rgb<u8>, looseness: f32) -> bool {
    assert!((0.0..=1.0).contains(&looseness));
    let max_diff = (looseness * 255.0) as u8;

    for (c1, c2) in zip(px1.0, px2.0) {
        if c1.abs_diff(c2) > max_diff {
            return false;
        }
    }

    true
}

pub fn screenshot(x: u16, y: u16, width: u16, height: u16) -> DynamicImage {
    if !on_screen(x + width, y + height) {
        panic!("One or more specified parameter is not within the screen size. Use screen::size() to check.")
    }
    let screen =
        Screen::from_point(x.into(), y.into()).expect("Cannot get screen from specified x and y");

    let capture = screen
        .capture_area(x.into(), y.into(), width.into(), height.into())
        .expect("Unable to screen capture.");

    let buffer = capture.buffer();

    image::load_from_memory(buffer).unwrap()
}
