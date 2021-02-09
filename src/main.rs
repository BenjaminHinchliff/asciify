use std::{io::Write, path::PathBuf, str::FromStr, thread, time};

use lazy_static::lazy_static;
use opencv::imgproc;
use opencv::{
    core::{Mat, Size},
    prelude::MatTrait,
};

use console::Term;

mod source;
use source::Source;

const CHARACTER_ASPECT: f32 = 1.0 / 2.6;

lazy_static! {
    static ref ACSII_CHARS: Vec<char> =
        "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. "
            .chars()
            .rev()
            .collect();
}

fn main() -> anyhow::Result<()> {
    let path = PathBuf::from_str("video/bad-apple.mp4")?;
    let mut source = Source::new(&path)?;

    let mut term = Term::buffered_stdout();
    term.clear_screen()?;

    let (height, _) = term.size();
    let height = height as i32;
    let width = (height as f32 * source.aspect_ratio()? / CHARACTER_ASPECT) as i32;
    let dest_size = Size::new(width, height);

    let source_fps = source.fps()?;
    let target_duration = time::Duration::from_secs_f32(1.0 / source_fps);
    loop {
        let start = time::Instant::now();
        let frame = match source.get_frame() {
            Ok(frame) => Ok(frame),
            Err(source::SourceError::OutOfFrames) => break,
            Err(err) => Err(err),
        }?;

        let mut downscaled = Mat::default()?;

        imgproc::resize(
            &frame,
            &mut downscaled,
            dest_size,
            0.0,
            0.0,
            imgproc::INTER_CUBIC,
        )?;

        let mut greyscale = Mat::default()?;
        imgproc::cvt_color(&downscaled, &mut greyscale, imgproc::COLOR_BGR2GRAY, 0)?;

        term.move_cursor_to(0, 0)?;
        for y in 0..height {
            for x in 0..width {
                write!(
                    term,
                    "{}",
                    ACSII_CHARS[*greyscale.at_2d::<std::os::raw::c_uchar>(y, x)? as usize
                        * ACSII_CHARS.len()
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
