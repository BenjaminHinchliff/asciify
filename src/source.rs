use std::path::Path;

use opencv::core::Mat;
use opencv::videoio::prelude::*;
use opencv::videoio::{self, VideoCapture};

#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error("failed to open source video file {0}")]
    UnableToOpenFile(String),
    #[error("opencv error {0}")]
    OpenCVError(#[from] opencv::Error),
    #[error("source ran out of frames")]
    OutOfFrames,
}

type SourceResult<T> = Result<T, SourceError>;

pub struct Source {
    capture: VideoCapture,
}

impl Source {
    pub fn new(file: &Path) -> SourceResult<Self> {
        let capture = VideoCapture::from_file(&file.to_string_lossy(), 0)?;
        let open = VideoCapture::is_opened(&capture)?;
        if !open {
            return Err(SourceError::UnableToOpenFile(
                file.to_string_lossy().to_string(),
            ));
        }
        Ok(Self { capture })
    }

    pub fn width(&self) -> SourceResult<i32> {
        Ok(self.capture.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32)
    }

    pub fn height(&self) -> SourceResult<i32> {
        Ok(self.capture.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32)
    }

    pub fn aspect_ratio(&self) -> SourceResult<f32> {
        Ok(self.width()? as f32 / self.height()? as f32)
    }

    pub fn fps(&self) -> SourceResult<f32> {
        Ok(self.capture.get(videoio::CAP_PROP_FPS)? as f32)
    }

    pub fn get_frame(&mut self) -> SourceResult<Mat> {
        let mut frame = Mat::default()?;
        let grabbed = self.capture.read(&mut frame)?;
        if !grabbed {
            return Err(SourceError::OutOfFrames);
        }
        Ok(frame)
    }
}
