use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

struct Palette {
    data: Vec<Rgba>,
}

impl Palette {
    fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut data = Vec::new();
        for color_index in 0..256 {
            let mut rgb = [0u8; 3];
            reader.read_exact(&mut rgb)?;
            let alpha = if color_index == 255 { 0 } else { 255 };
            data.push(Rgba {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
                a: alpha,
            });
        }

        Ok(Self { data })
    }

    pub fn get(&self, index: u8) -> Rgba {
        self.data[index as usize]
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
    index: Vec<u8>,
}

impl Image {
    pub fn load_ob5<P: AsRef<Path>>(
        path: P,
        size: Size,
        image_count: usize,
    ) -> std::io::Result<Vec<Self>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut result = Vec::new();
        let count = size.count();

        for _ in 0..image_count {
            let mut index = Vec::new();
            while index.len() < count {
                let mut input = [0u8; 2];
                reader.read_exact(&mut input)?;
                for _ in 0..input[0] {
                    index.push(input[1]);
                }
            }
            result.push(Self { size, index });
        }

        Ok(result)
    }

    pub fn save_tga<P: AsRef<Path>>(&self, path: P, palette: &Palette) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let wb = self.size.width.to_le_bytes();
        let hb = self.size.height.to_le_bytes();
        let header: [u8; 18] = [
            0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, wb[0], wb[1], hb[0], hb[1], 32, 0,
        ];
        writer.write_all(&header)?;

        for row in self.index.chunks(self.size.width as usize).rev() {
            for &color_index in row {
                let rgba = palette.get(color_index);
                let color = [rgba.b, rgba.g, rgba.r, rgba.a];
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
        let mut index = vec![0u8; size.count()];

        let mut left = usize_total_width;
        for image in images {
            let usize_image_width = image.size.width as usize;
            left -= usize_image_width;
            let mut dest = left;
            for row in image.index.chunks(usize_image_width) {
                index.splice(dest..(dest + usize_image_width), row.iter().copied());
                dest += usize_total_width;
            }
        }

        Image { size, index }
    }

    pub fn hit_map(&self) -> HitMap {
        let size = self.size;
        let mut map = Vec::with_capacity(size.count());
        for &color_index in self.index.iter() {
            map.push(color_index < 16);
        }
        HitMap { map }
    }
}

struct HitMap {
    map: Vec<bool>,
}

impl HitMap {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let mut last_hit = self.map[0];
        let mut count = 1u8;

        for &hit in self.map.iter().skip(1) {
            if last_hit == hit && count < 255 {
                count += 1;
            } else {
                let data = [count, if last_hit { 1 } else { 0 }];
                writer.write_all(&data)?;
                last_hit = hit;
                count = 1;
            }
        }

        let data = [count, if last_hit { 1 } else { 0 }];
        writer.write_all(&data)?;

        Ok(())
    }
}

fn main() {
    let palette = Palette::load("assets/vga.pal").expect("Unable to load palette");
    let images =
        Image::load_ob5("LEV4.KR3", Size::new(320, 192), 11).expect("Unable to load image");

    let level_path = format!("assets/levels/{}", 0);
    let joined = Image::join_horizontal(&images[0..10]);
    joined
        .save_tga(format!("{}/fg.tga", level_path), &palette)
        .expect("Unable to write output file");

    let hit_map = joined.hit_map();
    hit_map
        .save(format!("{}/map.hit", level_path))
        .expect("Unabel to save hit map");

    images[10]
        .save_tga(format!("{}/bcg.tga", level_path), &palette)
        .expect("Unable to save backgroudn");

    println!("Done");
}
