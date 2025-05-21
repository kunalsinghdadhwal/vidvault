use std::{fs, thread, vec};

use anyhow::{Error, Ok, anyhow};

use chrono::format;
use opencv::core::Mat;
use opencv::prelude::*;
use opencv::videoio::{CAP_ANY, VideoCapture, VideoWriter};

use crate::settings::{Data, OutputMode, Settings};
use crate::source::EmbedSource;
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

fn etch_pixel(frame: &mut EmbedSource, x: i32, y: i32, color: Vec<u8>) -> anyhow::Result<()> {
    for i in 0..frame.size {
        for j in 0..frame.size {
            let mut bgr = frame
                .image
                .at_2d_mut::<opencv::core::Vec3b>(y + i, x + j)
                .unwrap();

            bgr[2] = color[0];
            bgr[1] = color[1];
            bgr[0] = color[2];
        }
    }
    Ok(())
}

