use std::fs;
use std::io::prelude::*;

pub struct Cartridge {
    pub rom: [u8; 3584],
}

impl Cartridge {
    pub fn new(file: &str) -> Self {
        let mut f = fs::File::open(file).expect("file not found");
        let mut buffer = [0u8; 3584];
        let _ = f.read(&mut buffer);
        Cartridge { rom: buffer }
    }
}
