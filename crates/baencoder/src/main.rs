use std::{ffi::OsStr, fs, io::Cursor, path::Path};

use binrw::BinWrite;
use clap::{self, Parser};
use color_eyre::eyre::{OptionExt, Report, Result};
use video_rs::Decoder;

use crate::filedef::{BaFile, Frame, Section};

mod filedef;

/// Encode videos to .ba.
#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    /// Path to the video to encode.
    path: String,
    /// Use grayscale instead of black or white.
    #[arg(short, long)]
    grayscale: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    video_rs::init().map_err(|err| Report::msg(err.to_string()))?;

    let args = Cli::parse();

    let source = Path::new(&args.path);

    let file_name = source
        .file_stem()
        .unwrap_or(&OsStr::new("video"))
        .to_str()
        .unwrap_or("video");
    let target = source
        .parent()
        .ok_or_eyre("Provided file has no base directory")?
        .join(Path::new(&format!("{}.ba", file_name)));

    let mut decoder = Decoder::new(source)?;

    let (source_w, source_h) = decoder.size();
    let source_framerate = decoder.frame_rate().floor() as u8;

    println!(
        "Processing {}x{} frames at {} FPS.",
        source_w, source_h, source_framerate
    );

    let mut bafile = BaFile {
        frame_width: source_w as u8,
        frame_height: source_h as u8,
        frame_rate: source_framerate,
        frame_count: 0,
        frames: Vec::new(),
    };

    for frame in decoder.decode_iter() {
        if let Ok((_, frame)) = frame {
            let baframe = encode_frame(
                bafile.frame_width,
                bafile.frame_height,
                frame,
                args.grayscale,
            );
            bafile.frames.push(baframe);
        } else {
            break;
        }
    }

    bafile.frame_count = bafile.frames.len() as u32;

    let mut writer = Cursor::new(Vec::new());
    bafile.write_le(&mut writer)?;

    fs::write(target, writer.into_inner())?;
    Ok(())
}

fn encode_frame(width: u8, height: u8, frame: video_rs::Frame, to_grayscale: bool) -> Frame {
    let mut sections = Vec::new();

    let mut current_color = 0u8;
    let mut current_count = 0u16;
    for y in 0..height {
        for x in 0..width {
            let rgb = frame
                .slice(ndarray::s![y as usize, x as usize, ..])
                .to_slice()
                .unwrap();

            let grayscale = if to_grayscale {
                rgb_to_grayscale_avg(&rgb[0], &rgb[1], &rgb[2])
            } else {
                rgb_to_bw(&rgb[0], &rgb[1], &rgb[2])
            };
            if grayscale == current_color {
                current_count += 1;
            } else {
                if current_count > 0 {
                    sections.push(Section::new(current_color, current_count));
                }
                current_color = grayscale;
                current_count = 1;
            }
        }
    }

    if current_count > 0 {
        sections.push(Section::new(current_color, current_count))
    }

    Frame::new(sections)
}

fn rgb_to_grayscale_avg(r: &u8, g: &u8, b: &u8) -> u8 {
    ((r.clone() as u32 + g.clone() as u32 + b.clone() as u32) / 3) as u8
}

fn rgb_to_bw(r: &u8, g: &u8, b: &u8) -> u8 {
    let grayscale = rgb_to_grayscale_avg(r, g, b);
    if grayscale > 128 { 0xFF } else { 0x00 }
}
