use std::{path::Path, fs::File, io::{BufReader, Read}};

#[repr(C)]
#[derive(Clone, Copy)]
struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Rgb {
    const BLACK: Rgb = Rgb {r: 0, g: 0, b: 0};

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {r, g, b}
    }

    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}

struct Palette {
    data: [Rgb; 256]
}

impl Palette {
    fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {        
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut data = [Rgb::BLACK; 256];
        for color in &mut data {
            let mut rgb = [0u8; 3];
            reader.read_exact(&mut rgb)?;
            color.set(rgb[0], rgb[1], rgb[2]);
        }

        Ok(Self {data})
    }
}




fn main() {
    let palette = Palette::load("assets/vga.pal").unwrap();
    println!("OK"); //, std::env::current_dir().unwrap());
}