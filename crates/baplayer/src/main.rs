use std::{
    fs::File,
    io::{BufReader, Read, Write, stdout},
    thread,
    time::{Duration, SystemTime},
};

use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Parser)]
struct Cli {
    path: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    let file = File::open(args.path)?;

    let mut seeker = BufSeeker::new(file);

    let height = seeker.read_u8()?;
    let width = seeker.read_u8()?;
    let framerate = seeker.read_u8()?;
    let frame_count = seeker.read_u32()?;
    let frame_time = Duration::from_nanos(1000000000 / framerate as u64);

    for _ in 0..frame_count {
        let start_t = SystemTime::now();

        let mut decoded_frame = Vec::new();
        let section_count = seeker.read_u16()?;

        for _ in 0..section_count {
            let color = seeker.read_u8()?;
            let pixel_count = seeker.read_u16()? as usize;
            decoded_frame.append(&mut vec![color; pixel_count]);
        }

        let mut frame = "\x1B[2J".to_owned();
        for y in 0..height / 4 {
            for x in 0..width / 2 {
                let start_pos = width as usize * y as usize * 4 + x as usize * 2;
                frame.push(
                    get_unicode_char(&[
                        decoded_frame[start_pos],
                        decoded_frame[start_pos + 1],
                        decoded_frame[start_pos + width as usize],
                        decoded_frame[start_pos + width as usize + 1],
                        decoded_frame[start_pos + width as usize * 2],
                        decoded_frame[start_pos + width as usize * 2 + 1],
                        decoded_frame[start_pos + width as usize * 3],
                        decoded_frame[start_pos + width as usize * 3 + 1],
                    ])
                    .expect("msg"),
                )
            }
            frame.push('\n');
        }
        let mut lock = stdout().lock();
        write!(lock, "{}", frame).unwrap();
        let duration = SystemTime::now().duration_since(start_t).unwrap();
        thread::sleep(frame_time - duration);
    }

    Ok(())
}

pub struct BufSeeker {
    reader: BufReader<File>,
}

impl BufSeeker {
    pub fn new(source: File) -> Self {
        Self {
            reader: BufReader::new(source),
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

fn int(b: u8) -> u32 {
    if b == 0x0 {
        return 0;
    } else {
        return 1;
    }
}

pub fn get_unicode_char(input: &[u8; 8]) -> Option<char> {
    let hex: u32 = int(input[0]) * 0x1
        + int(input[1]) * 0x8
        + int(input[2]) * 0x2
        + int(input[3]) * 0x10
        + int(input[4]) * 0x4
        + int(input[5]) * 0x20
        + int(input[6]) * 0x40
        + int(input[7]) * 0x80;

    return char::from_u32(0x2800 + hex);
}
