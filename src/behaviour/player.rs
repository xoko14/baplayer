use std::{io::{BufReader, SeekFrom, Seek, Read, stdout, Write, Error}, fs::File, time::{Duration, SystemTime}, thread};

pub struct Player{
    height: usize,
    width: usize,
    framerate: u64,
    file: File
}

impl Player{
    pub fn new(height: usize, width: usize, framerate: u64, file: File) -> Self{
        Player{
            height, width, framerate, file
        }
    }

    pub fn play(&self) -> Result<(), Error>{
        let bpf: usize = self.width*self.height;
        let mut reader = BufReader::new(&self.file);
        let mut buffer: Vec<u8>= vec![0; bpf];
        let frame_time = Duration::from_nanos(1000000000/self.framerate);
        let total_frames = usize::try_from(self.file.metadata().unwrap().len() / u64::try_from(bpf).unwrap()).unwrap();

        for i in 0..total_frames {
            let start_t = SystemTime::now();
            reader.seek(SeekFrom::Start(((self.width * self.height * i) as u64).try_into().expect("msg")))?;
            reader.read_exact(&mut buffer)?;

            let mut frame = "".to_owned();
            for y in 0..self.height/4 {
                for x in 0..self.width/2{
                    let start_pos = self.width * y * 4 + x * 2;
                    frame.push(
                        crate::braille::get_unicode_char(&[
                            buffer[start_pos],
                            buffer[start_pos + 1],
                            buffer[start_pos + self.width],
                            buffer[start_pos + self.width + 1],
                            buffer[start_pos + self.width * 2],
                            buffer[start_pos + self.width * 2 + 1],
                            buffer[start_pos + self.width * 3],
                            buffer[start_pos + self.width * 3 + 1],
                        ])
                        .expect("msg"),
                    )
                }
                frame.push('\n');
            }
            let mut lock = stdout().lock();
            write!(lock, "{}", frame).unwrap();
            let duration = SystemTime::now().duration_since(start_t).unwrap();
            thread::sleep(frame_time-duration);
        }
        Ok(())
    }
}