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
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    const BLACK: Rgba = Rgba {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub fn from_rgb(rgb: Rgb, a: u8) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a,
        }
    }
}

struct Palette {
    data: Vec<Rgb>,
}

impl Palette {
    fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut data = Vec::new();
        for _ in 0..256 {
            let mut rgb = [0u8; 3];
            reader.read_exact(&mut rgb)?;
            data.push(Rgb::new(rgb[0], rgb[1], rgb[2]));
        }

        Ok(Self { data })
    }

    pub fn get(&self, index: usize) -> Rgb {
        self.data[index]
    }
}

#[derive(Clone, Copy, Debug)]
struct Size {
    width: u32,
    height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn count(&self) -> usize {
        (self.width * self.height) as usize
    }
}

struct Image {
    size: Size,
    data: Vec<Rgba>,
}

impl Image {
    pub fn load_ob5<P: AsRef<Path>>(
        path: P,
        size: Size,
        image_count: usize,
        palette: &Palette,
    ) -> std::io::Result<Vec<Self>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut result = Vec::new();
        let count = size.count();

        for _ in 0..image_count {
            let mut data = Vec::new();
            while data.len() < count {
                let mut input = [0u8; 2];
                reader.read_exact(&mut input)?;
                let color = palette.get(input[1] as usize);
                for _ in 0..input[0] {
                    let alpha = if input[1] == 255 { 0 } else { 255 };
                    data.push(Rgba::from_rgb(color, alpha));
                }
            }
            result.push(Self { size, data });
        }

        Ok(result)
    }

    pub fn save_tga<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let wb = self.size.width.to_le_bytes();
        let hb = self.size.height.to_le_bytes();
        let header: [u8; 18] = [
            0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, wb[0], wb[1], hb[0], hb[1], 32, 0,
        ];
        writer.write_all(&header)?;

        for row in self.data.chunks(self.size.width as usize).rev() {
            for rgb in row {
                let color = [rgb.b, rgb.g, rgb.r, rgb.a];
                writer.write_all(&color)?;
            }
        }

        Ok(())
    }

    pub fn join_horizontal(images: &[Image]) -> Image {
        let width = images
            .iter()
            .fold(0u32, |width, image| width + image.size.width);
        let size = Size::new(width, images[0].size.height);
        let usize_total_width = size.width as usize;
        let mut data = vec![Rgba::BLACK; size.count()];

        let mut left = usize_total_width;
        for image in images {
            let usize_image_width = image.size.width as usize;
            left -= usize_image_width;
            let mut dest = left;
            for row in image.data.chunks(usize_image_width) {
                data.splice(dest..(dest + usize_image_width), row.iter().copied());
                dest += usize_total_width;
            }
        }

        Image { size, data }
    }
}

fn main() {
    let palette = Palette::load("assets/vga.pal").expect("Unable to load palette");
    let images = Image::load_ob5("lev1.kr3", Size::new(320, 192), 11, &palette)
        .expect("Unable to load image");
    let joined = Image::join_horizontal(&images[0..10]);
    joined
        .save_tga("lev1.tga")
        .expect("Unable to write output file");
    images[10]
        .save_tga("lev1bcg.tga")
        .expect("Unable to save backgroudn");
    println!("Done");
}
