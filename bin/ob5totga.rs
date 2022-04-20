use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };

    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}

struct Palette {
    data: [Rgb; 256],
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

        Ok(Self { data })
    }

    pub fn get(&self, index: usize) -> Rgb {
        self.data[index]
    }
}

struct Size {
    width: u32,
    height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

struct Image {
    size: Size,
    data: Vec<Rgb>,
}

impl Image {
    pub fn load_ob5<P: AsRef<Path>>(
        path: P,
        size: Size,
        palette: &Palette,
    ) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let count = (size.width * size.height) as usize;
        let mut data = Vec::new();
        while data.len() < count {
            let mut input = [0u8; 2];
            reader.read_exact(&mut input)?;
            let color = palette.get(input[1] as usize);
            for _ in 0..input[0] {
                data.push(color);
            }
        }

        Ok(Self { size, data })
    }

    pub fn save_tga<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let wb = self.size.width.to_le_bytes();
        let hb = self.size.height.to_le_bytes();
        let header: [u8; 18] = [
            0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, wb[0], wb[1], hb[0], hb[1], 24, 0,
        ];
        writer.write_all(&header)?;

        for row in self.data.chunks(self.size.width as usize).rev() {
            for rgb in row {
                let color = [rgb.b, rgb.g, rgb.r];
                writer.write_all(&color)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let palette = Palette::load("assets/vga.pal").expect("Unable to load palette");
    let image =
        Image::load_ob5("lev1.kr3", Size::new(320, 192), &palette).expect("Unable to load image");
    image
        .save_tga("lev1.tga")
        .expect("Unable to write output file");
    println!("Done");
}
