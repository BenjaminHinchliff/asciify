use std::{io::Write, thread, time};

use opencv::imgproc;
use opencv::videoio::prelude::*;
use opencv::videoio::{self, VideoCapture};
use opencv::{
    core::{Mat, Size},
    prelude::MatTrait,
};

use console::Term;

mod source;

fn main() -> anyhow::Result<()> {
    let mut capture = VideoCapture::from_file("video/test.mp4", 0)?;
    let open = VideoCapture::is_opened(&capture)?;
    if !open {
        panic!("failed to open input file");
    }

    let mut term = Term::buffered_stdout();
    term.clear_screen()?;

    let source_height = capture.get(videoio::CAP_PROP_FRAME_WIDTH)?;
    let source_width = capture.get(videoio::CAP_PROP_FRAME_WIDTH)?;
    let source_aspect = source_width / source_height;
    let (height, _) = term.size();
    let height = height as i32;
    let width = (height as f64 * source_aspect / CHARACTER_ASPECT) as i32;
    let dest_size = Size::new(width, height);

    let source_fps = capture.get(videoio::CAP_PROP_FPS)?;
    let target_duration = time::Duration::from_secs_f64(1.0 / source_fps);
    loop {
        let start = time::Instant::now();
        let mut frame = Mat::default()?;
        let grabbed = capture.read(&mut frame)?;

        if !grabbed {
            break;
        }

        let mut downscaled = Mat::default()?;

        imgproc::resize(&frame, &mut downscaled, dest_size, 0.0, 0.0, imgproc::INTER_LINEAR)?;

        let mut greyscale = Mat::default()?;
        imgproc::cvt_color(&downscaled, &mut greyscale, imgproc::COLOR_BGR2GRAY, 0)?;

        term.move_cursor_to(0, 0)?;
        for y in 0..height {
            for x in 0..width {
                write!(
                    term,
                    "{}",
                    acsii_chars[*greyscale.at_2d::<std::os::raw::c_uchar>(y, x)? as usize
                        * acsii_chars.len()
                        / 256]
                )?;
            }
            if y != height - 1 {
                writeln!(term)?;
            }
        }
        term.flush()?;
        let draw_time = time::Instant::now() - start;
        if draw_time < target_duration {
            thread::sleep(target_duration - draw_time);
        }
    }

    Ok(())
}
