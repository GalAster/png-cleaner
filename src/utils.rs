use crate::{Config, Error};
use glob::GlobResult;
use png::{BitDepth, ColorType, OutputInfo};
use std::{fs, fs::File, io::Write};

#[derive(Clone, Debug)]
pub struct PNG {
    pub path: Box<str>,
    pub size: u64,
    pub ratio: f32,
}

impl PNG {
    pub fn new(path: &str) -> Self {
        Self { path: Box::from(path), size: 0, ratio: -1.0 }
    }

    pub fn size_ratio(&mut self) -> Result<(), Error> {
        let (info, _) = png::Decoder::new(File::open(&*self.path)?).read_info()?;
        self.size = fs::metadata(&*self.path)?.len();
        self.ratio = *&self.size as f32 / estimate_size(&info);
        return Ok(());
    }
}

pub fn estimate_size(info: &OutputInfo) -> f32 {
    let w = info.width as f32;
    let h = info.height as f32;
    let b = match info.bit_depth {
        BitDepth::One => 0.125,
        BitDepth::Two => 0.25,
        BitDepth::Four => 0.5,
        BitDepth::Eight => 1.0,
        BitDepth::Sixteen => 2.0,
    };
    let d = match info.color_type {
        ColorType::Grayscale => 1.0,
        ColorType::RGB => 3.0,
        ColorType::Indexed => 1.0,
        ColorType::GrayscaleAlpha => 1.0,
        ColorType::RGBA => 4.0,
    };
    return w * h * b * d;
}

pub fn write_to_file(path: &str, bytes: &[u8]) -> Result<(), Error> {
    println!("Generating {}", path);
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    Ok(())
}

pub fn check_file(entry: GlobResult, cfg: &Config) -> Result<String, Error> {
    let path = &entry?.to_path_buf();
    let (info, _) = png::Decoder::new(File::open(path)?).read_info()?;
    let size = fs::metadata(path)?.len();
    let ratio = size as f32 / estimate_size(&info);
    if size > cfg.min_size || ratio > cfg.min_ratio {
        //println!("{:?}", PNG { path: Box::from(path.to_str()?), size, ratio });
        return Ok(format!("{},{},{}\n", path.to_str().unwrap_or_default(), size / 1024, ratio));
    }
    else {
        Err(Error::UnknownIOError)
    }
}
