use serde::Deserialize;

pub enum OutputMode {
    Color,
    Binary,
}

pub struct Data {
    pub bytes: Vec<u8>,
    pub binary: Vec<bool>,
    pub out_mode: OutputMode,
}

impl Data {
    pub fn new_out_mode(out_mode: OutputMode) -> Self {
        Data {
            bytes: Vec::new(),
            binary: Vec::new(),
            out_mode,
        }
    }

    pub fn from_binary(binary: Vec<bool>) -> Self {
        Data {
            bytes: Vec::new(),
            binary,
            out_mode: OutputMode::Binary,
        }
    }

    pub fn from_color(bytes: Vec<u8>) -> Self {
        Data {
            bytes,
            binary: Vec::new(),
            out_mode: OutputMode::Color,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct Settings {
    pub size: i32,

    pub threads: usize,

    pub width: i32,

    pub fps: f64,

    pub height: i32,
}

impl Settings {
    pub fn new(size: i32, threads: usize, fps: i32, width: i32, height: i32) -> Self {
        Settings {
            size,
            threads,
            fps: fps as f64,
            width,
            height,
        }
    }
}
