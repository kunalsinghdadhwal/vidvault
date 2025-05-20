use opencv::core::prelude::*;
use opencv::core::{CV_8UC3, Mat, Size, Size_};

pub struct EmbedSource {
    pub image: Mat,

    pub size: i32,

    pub frame_size: Size,

    pub actual_size: Size,
}

impl EmbedSource {
    pub fn new(size: i32, width: i32, height: i32) -> Self {
        let frame_size: Size_<i32> = Size::new(width, height);
        let actual_width: i32 = width - (width % size);
        let actual_height: i32 = height - (height % size);
        let actual_size: Size_<i32> = Size::new(actual_width, actual_height);

        unsafe {
            let image: Mat = Mat::new_rows_cols(frame_size.height, frame_size.width, CV_8UC3)
                .expect("Failed to create image");

            EmbedSource {
                image,
                size,
                frame_size,
                actual_size,
            }
        }
    }

    pub fn from(image: Mat, size: i32, instruction: bool) -> Result<EmbedSource, String> {
        let width = image.cols();
        let height = image.rows();

        let frame_size: Size_<i32> = Size::new(width, height);

        if height % size != 0 && !instruction {
            return Err("Image size is not a multiple of the embedding size".to_string());
        }

        let adjusted_width: i32 = width - (width % size);
        let adjusted_height: i32 = height - (height % size);

        let actual_size: Size_<i32> = Size::new(adjusted_width, adjusted_height);

        Ok(EmbedSource {
            image,
            size,
            frame_size,
            actual_size,
        })
    }
}
