use std::path::Path;

use lazy_static::lazy_static;

use opencv::imgproc;
use opencv::videoio::prelude::*;
use opencv::videoio::{self, VideoCapture};
use opencv::{
    core::{Mat, Size},
    prelude::MatTrait,
};

const CHARACTER_ASPECT: f64 = 1.0 / 2.6;

lazy_static! {
    static ref ASCII_CHARS: Vec<char> =
        " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"
            .chars()
            .collect();
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SourceError {
    #[error("failed to open source video file {0}")]
    UnableToOpenFile(String),
}

pub struct Source {
    capture: VideoCapture,
}

impl Source {
    pub fn new(file: &Path) -> Result<Self, SourceError> {
        let mut capture = VideoCapture::from_file(&file.to_string_lossy(), 0)?;
        let open = VideoCapture::is_opened(&capture)?;
        if !open {
            return Err(SourceError::UnableToOpenFile(file.to_string_lossy().to_string()));
        }
        Ok(Self {
            capture,
        })
    }
}
