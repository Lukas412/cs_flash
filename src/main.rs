use anyhow::anyhow;
use enigo::{Enigo, Key, KeyboardControllable};
use win_streamshot::{Screenshot, WindowFinder};

const WHITE: u32 = 80;
const TOLERANCE: u32 = 1;

const CHECKED_PIXEL_COUNT: u32 = 100;

fn main() -> anyhow::Result<()> {
    let window_finder = WindowFinder::new()?;
    let mut csgo_screenshot_buffer = window_finder
        .find("Counter-Strike")
        .ok_or_else(|| anyhow!("please start counter strike before this script is running"))??;

    let mut currently_flash_banged = false;
    let mut enigo = Enigo::new();
    loop {
        let screenshot = csgo_screenshot_buffer.get_bgr_screenshot()?;
        if !is_flash_bang(screenshot) {
            if currently_flash_banged {
                currently_flash_banged = false;
                enigo.key_up(Key::F13);
                println!("flash bang ended");
            }
            continue;
        }
        if !currently_flash_banged {
            currently_flash_banged = true;
            enigo.key_down(Key::F13);
            println!("flash bang started");
        }
    }
}

fn is_flash_bang<Color>(screenshot: Screenshot<Color>) -> bool {
    let skip_size = screenshot.total_pixels() / CHECKED_PIXEL_COUNT;
    let (grayscale_total, span_total) = screenshot
        .chunks_exact(4)
        .enumerate()
        .filter_map(|(index, pixel)| (index % skip_size as usize == 0).then_some(pixel))
        .map(|pixel| {
            let greyscale = (pixel[0] + pixel[1] + pixel[2]) / 3;
            let span = (pixel[0].abs_diff(greyscale)
                + pixel[0].abs_diff(greyscale)
                + pixel[2].abs_diff(greyscale))
                / 3;
            (greyscale, span)
        })
        .fold((0, 0), |(grayscale_sum, span_sum), (grayscale, span)| {
            (grayscale_sum + grayscale as u32, span_sum + span as u32)
        });

    let grayscale_average = grayscale_total / CHECKED_PIXEL_COUNT;
    let span_average = span_total / CHECKED_PIXEL_COUNT;

    grayscale_average > WHITE && span_average <= TOLERANCE
}
