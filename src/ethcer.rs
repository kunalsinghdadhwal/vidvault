use std::{fs, i32, thread, u32, vec};

use anyhow::{Error, Ok, anyhow};

use chrono::format;
use opencv::core::Mat;
use opencv::prelude::*;
use opencv::videoio::{CAP_ANY, VideoCapture, VideoWriter};

use crate::settings::{self, Data, OutputMode, Settings};
use crate::source::{self, EmbedSource};
use crate::timer::Timer;

pub fn rip_bytes(path: &str) -> anyhow::Result<Vec<u8>> {
    let byte_data: Vec<u8> = fs::read(path)?;

    if byte_data.is_empty() {
        return Err(anyhow!("Empty files cannot be embedded in video"));
    }
    println!("Bytes Ripped Successfully");
    println!("Read {} bytes from {}", byte_data.len(), path);
    Ok(byte_data)
}

pub fn rip_binary(byte_data: Vec<u8>) -> anyhow::Result<Vec<bool>> {
    let mut binary_data: Vec<bool> = Vec::new();
    for byte in byte_data {
        let mut bits: String = format!("{:b}", byte);
        let missing_0 = 8 - bits.len();

        for _ in 0..missing_0 {
            bits.insert(0, '0');
        }

        for bit in bits.chars() {
            binary_data.push(bit == '1');
        }
    }

    println!("Binary Ripped Successfully");
    println!(
        "Converted {} bytes to {} bits",
        byte_data.len(),
        binary_data.len()
    );
    Ok(binary_data)
}

pub fn rip_binary_u32(bytes: Vec<u32>) -> anyhow::Result<Vec<bool>> {
    let mut binary_data: Vec<bool> = Vec::new();
    for byte in bytes {
        let mut bits: String = format!("{:b}", byte);
        let missing_0 = 32 - bits.len();

        for _ in 0..missing_0 {
            bits.insert(0, '0');
        }

        for bit in bits.chars() {
            binary_data.push(bit == '1');
        }
    }

    println!("Binary Ripped Successfully");
    println!(
        "Converted {} bytes to {} bits",
        bytes.len(),
        binary_data.len()
    );
    Ok(binary_data)
}

fn translate_u8(binary_data: Vec<bool>) -> anyhow::Result<Vec<u8>> {
    let mut buffer: Vec<bool> = Vec::new();
    let mut byte_data: Vec<u8> = Vec::new();

    for bit in binary_data {
        buffer.push(bit);

        if buffer.len() == 8 {
            let byte: u8 = buffer.iter().fold(0u8, |v, b| (v << 1) + (*b as u8));
            byte_data.push(byte);
            buffer.clear();
        }
    }

    Ok(byte_data)
}

fn translate_u32(binary_data: Vec<bool>) -> anyhow::Result<Vec<u32>> {
    let mut buffer: Vec<bool> = Vec::new();
    let mut byte_data: Vec<u32> = Vec::new();

    for bit in binary_data {
        buffer.push(bit);

        if buffer.len() == 32 {
            let byte: u32 = buffer.iter().fold(0u32, |v, b| (v << 1) + (*b as u32));
            byte_data.push(byte);
            buffer.clear();
        }
    }

    Ok(byte_data)
}

pub fn write_bytes(path: &str, data: Vec<u8>) -> anyhow::Result<()> {
    fs::write(path, data)?;
    println!("File Written Successfully");
    println!("Wrote {} bytes to {}", data.len(), path);
    Ok(())
}

fn get_pixel(frame: &EmbedSource, x: i32, y: i32) -> Option<Vec<u8>> {
    let mut r_list: Vec<u8> = Vec::new();
    let mut g_list: Vec<u8> = Vec::new();
    let mut b_list: Vec<u8> = Vec::new();

    for i in 0..frame.size {
        for j in 0..frame.size {
            let bgr = frame
                .image
                .at_2d::<opencv::core::Vec3b>(y + i, x + j)
                .unwrap();

            r_list.push(bgr[2]);
            g_list.push(bgr[1]);
            b_list.push(bgr[0]);
        }
    }

    let r_avg = r_list.iter().map(|&x| x as usize).sum::<usize>() / r_list.len();
    let g_avg = g_list.iter().map(|&x| x as usize).sum::<usize>() / g_list.len();
    let b_avg = b_list.iter().map(|&x| x as usize).sum::<usize>() / b_list.len();

    Some(vec![r_avg as u8, g_avg as u8, b_avg as u8])
}

fn etch_pixel(frame: &mut EmbedSource, x: i32, y: i32, rgb: Vec<u8>) -> anyhow::Result<()> {
    for i in 0..frame.size {
        for j in 0..frame.size {
            let mut bgr = frame
                .image
                .at_2d_mut::<opencv::core::Vec3b>(y + i, x + j)
                .unwrap();

            bgr[2] = rgb[0];
            bgr[1] = rgb[1];
            bgr[0] = rgb[2];
        }
    }
    Ok(())
}

fn etch_color(
    source: &mut EmbedSource,
    data: &Vec<u8>,
    global_index: &mut usize,
) -> anyhow::Result<()> {
    let _timer = Timer::new("Etching frame");

    let widht = source.actual_size.width;
    let height = source.actual_size.height;
    let size = source.size as usize;

    for y in (0..height).step_by(size) {
        for x in (0..widht).step_by(size) {
            let local_idx = global_index.clone();

            let rgb = vec![data[local_idx], data[local_idx + 1], data[local_idx + 2]];

            etch_pixel(source, x, y, rgb).unwrap();
            *global_index += 3;

            if *global_index + 2 >= data.len() {
                return Err(Error::msg("Index Beyond Data"));
            }
        }
    }

    Ok(())
}

fn etch_bw(
    source: &mut EmbedSource,
    data: &Vec<bool>,
    global_index: &mut usize,
) -> anyhow::Result<()> {
    let _timer = Timer::new("Etching frame");

    let widht = source.actual_size.width;
    let height = source.actual_size.height;
    let size = source.size as usize;

    for y in (0..height).step_by(size) {
        for x in (0..widht).step_by(size) {
            let local_idx = global_index.clone();

            let brightness = if data[local_idx] { 255 } else { 0 };

            let rgb = vec![brightness, brightness, brightness];
            etch_pixel(source, x, y, rgb).unwrap();
            *global_index += 1;
            if *global_index >= data.len() {
                return Err(Error::msg("Index Beyond Data"));
            }
        }
    }

    Ok(())
}

fn read_bw(
    source: &EmbedSource,
    current_frame: i32,
    final_frame: i32,
    final_bit: i32,
) -> anyhow::Result<Vec<bool>> {
    let width: i32 = source.actual_size.width;
    let height: i32 = source.actual_size.height;

    let size = source.size as usize;
    let mut binary_data: Vec<bool> = Vec::new();

    for y in (0..height).step_by(size) {
        for x in (0..width).step_by(size) {
            let rgb = get_pixel(&source, x, y);

            if rgb.is_none() {
                continue;
            } else {
                let rgb = rgb.unwrap();

                if rgb[0] >= 127 {
                    binary_data.push(true);
                } else {
                    binary_data.push(false);
                }
            }
        }
    }

    if current_frame == final_frame {
        let slice = binary_data[0..final_bit as usize].to_vec();
        return Ok(slice);
    }

    Ok(binary_data)
}

fn read_color(
    source: &EmbedSource,
    current_frame: i32,
    final_frame: i32,
    final_bit: i32,
) -> anyhow::Result<Vec<u8>> {
    let width: i32 = source.actual_size.width;
    let height: i32 = source.actual_size.height;

    let size = source.size as usize;
    let mut byte_data: Vec<u8> = Vec::new();

    for y in (0..height).step_by(size) {
        for x in (0..width).step_by(size) {
            let rgb = get_pixel(&source, x, y);

            if rgb.is_none() {
                continue;
            } else {
                let rgb = rgb.unwrap();
                byte_data.push(rgb[0]);
                byte_data.push(rgb[1]);
                byte_data.push(rgb[2]);
            }
        }
    }

    if current_frame == final_frame {
        let slice = byte_data[0..final_bit as usize].to_vec();
        return Ok(slice);
    }

    Ok(byte_data)
}

fn etch_instructions(settings: &Settings, data: &Data) -> anyhow::Result<EmbedSource> {
    let instruction_size = 5;

    let mut u32_instructions: Vec<u32> = Vec::new();

    let frame_size = (settings.width * settings.height) as usize;

    match data.out_mode {
        OutputMode::Color => {
            u32_instructions.push(u32::MAX);

            let frame_data_size = frame_size / settings.size.pow(2) as usize;
            let final_byte = data.bytes.len() % frame_data_size;
            let mut final_frame = data.bytes.len() / frame_data_size;

            if data.bytes.len() % frame_size != 0 {
                final_frame += 1;
            }

            dbg!(final_frame);
            u32_instructions.push(final_frame as u32);
            u32_instructions.push(final_byte as u32);
        }
        OutputMode::Binary => {
            u32_instructions.push(u32::MIN);

            let frame_data_size = frame_size / settings.size.pow(2) as usize;
            let final_byte = data.binary.len() % frame_data_size;
            let mut final_frame = data.binary.len() / frame_data_size;

            if data.binary.len() % frame_size != 0 {
                final_frame += 1;
            }

            dbg!(final_frame);
            u32_instructions.push(final_frame as u32);
            u32_instructions.push(final_byte as u32);
        }
    }

    u32_instructions.push(settings.size as u32);
    u32_instructions.push(u32::MAX);

    let instruction_data = rip_binary_u32(u32_instructions)?;

    let mut source = EmbedSource::new(instruction_size, settings.width, settings.height);
    let mut index = 0;

    match etch_bw(&mut source, &instruction_data, &mut index) {
        Ok(_) => {}
        Err(_) => {
            println!("Instructions written successfully");
        }
    }

    Ok(source)
}

fn read_instructions(
    source: &EmbedSource,
    threads: usize,
) -> anyhow::Result<(OutputMode, i32, i32, Settings)> {
    let binary_data = read_bw(source, 0, 1, 0)?;
    let u32_data = translate_u32(binary_data)?;

    let out_mode = match u32_data[0] {
        u32::MAX => OutputMode::Color,
        _ => OutputMode::Binary,
    };

    let final_frame = u32_data[1] as i32;
    let final_bit = u32_data[2] as i32;
    let size = u32_data[3] as usize;

    let height = source.frame_size.height;
    let width = source.frame_size.width;

    let settings = Settings::new(size, threads, 1337, width, height);

    Ok((out_mode, final_frame, final_bit, settings))
}

pub fn etch(path: &str, data: Data, settings: Settings) -> anyhow::Result<()> {
    let _timer = Timer::new("Etching video");

    let mut spool = Vec::new();

    match data.out_mode {
        OutputMode::Color => {
            let length = data.bytes.len();

            let frame_size = (settings.width * settings.height) as usize;
            let frame_data_size = frame_size / settings.size.pow(2) as usize * 3;
            let frame_length = length / frame_data_size;
            let chunk_frame_size = (frame_length / settings.threads) + 1;
            let chunk_data_size = chunk_frame_size * frame_data_size;

            for chunk in data.bytes.chunks(chunk_data_size) {
                let chunk_copy = chunk.to_vec();

                let thread = thread::spawn(move || {
                    let mut frames = Vec::new();
                    let mut index: usize = 0;

                    loop {
                        let mut source =
                            EmbedSource::new(settings.size, settings.width, settings.height);
                        match etch_color(&mut source, &chunk_copy, &mut index) {
                            Ok(_) => frames.push(source),
                            Err(_) => {
                                frames.push(source);
                                println!("Embedding Thread Finished!");
                                break;
                            }
                        }
                    }
                    frames
                });

                spool.push(thread);
            }
        }
        OutputMode::Binary => {
            let length = data.binary.len();

            let frame_size = (settings.width * settings.height) as usize;
            let frame_data_size = frame_size / settings.size.pow(2) as usize;
            let frame_length = length / frame_data_size;
            let chunk_frame_size = (frame_length / settings.threads) + 1;
            let chunk_data_size = chunk_frame_size * frame_data_size;

            for chunk in data.binary.chunks(chunk_data_size) {
                let chunk_copy = chunk.to_vec();

                let thread = thread::spawn(move || {
                    let mut frames = Vec::new();
                    let mut index: usize = 0;

                    loop {
                        let mut source =
                            EmbedSource::new(settings.size, settings.width, settings.height);
                        match etch_bw(&mut source, &chunk_copy, &mut index) {
                            Ok(_) => frames.push(source),
                            Err(_) => {
                                frames.push(source);
                                println!("Embedding Thread Finished!");
                                break;
                            }
                        }
                    }
                    frames
                });

                spool.push(thread);
            }
        }
    }

    let mut complete_frames = Vec::new();
    let instructional_frame = etch_instructions(&settings, &data)?;
    complete_frames.push(instructional_frame);

    for thread in spool {
        let frames = thread.join().unwrap();
        complete_frames.extend(frames);
    }

    let fourcc = VideoWriter::fourcc('p', 'n', 'g', ' ')?;

    let frame_size = complete_frames[1].frame_size;
    let mut video = VideoWriter::new(path, fourcc, settings.fps, frame_size, true);

    let mut video = match video {
        Ok(v) => v,
        Err(_) => {
            let fourcc = VideoWriter::fourcc('a', 'v', 'c', '1')?;
            VideoWriter::new(path, fourcc, settings.fps, frame_size, true)
                .expect("Both PNG and AVC1 codecs failed")
        }
    };

    for frame in complete_frames {
        let image = frame.image;
        video.write(&image).unwrap();
    }
    video.release().unwrap();
    println!("Video Etched Successfully at {}", path);
    printnl!("Time taken to etch video: {:?}", _timer.elapsed());
    Ok(())
}

pub fn read(path: &str, threads: usize) -> anyhow::Result<Vec<u8>> {
    let _timer = Timer::new("Dislodging video");
    const INSTRUCTION_SIZE: i32 = 5;

    let mut video = VideoCapture::from_file(path, CAP_ANY).expect("Could not open video path");
    let mut frame = Mat::default();

    video.read(&mut frame).unwrap();
    let instruction_source = EmbedSource::from(frame.clone(), INSTRUCTION_SIZE, true)
        .expect("Could not create instruction source");
    let (out_mode, final_frame, final_byte, settings) =
        read_instructions(&instruction_source, threads)?;

    let mut byte_data = Vec::new();
    let mut current_frame = 1;

    while video.read(&mut frame).unwrap() && frame.cols() > 0 {
        if current_frame % 20 == 0 {
            println!("Reading frame {}", current_frame);
        }

        let source =
            EmbedSource::from(frame.clone(), settings.size, false).expect("Reading frame failed");

        let frame_data = match out_mode {
            OutputMode::Color => read_color(&source, current_frame, i32::MAX, final_byte)
                .expect("Failed to read color frame"),
            OutputMode::Binary => {
                let binary_data = read_bw(&source, current_frame, final_frame, final_byte)
                    .expect("Failed to read binary frame");
                translate_u8(binary_data).expect("Failed to translate binary data")
            }
        };

        byte_data.extend(frame_data);
        current_frame += 1;
    }

    println!("Time taken to read video: {:?}", _timer.elapsed());
    println!("Video read successfully");
    Ok(byte_data)
}
