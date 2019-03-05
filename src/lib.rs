use {
    png::{Encoder, HasParameters},
    std::io::Write,
};

pub use failure::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color(pub [u8; 4]);

impl Color {
    pub const fn new() -> Self {
        Color([0, 0, 0, 0])
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color([r, g, b, a])
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new()
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            data: vec![Color::new(); (width * height) as usize],
        }
    }

    fn coord(&self, x: u32, y: u32) -> usize {
        (self.width * y + x) as usize
    }

    pub fn set(&mut self, x: u32, y: u32, color: Color) -> &mut Self {
        if x >= self.width || y >= self.height {
            panic!("invalid coordinate x: {}, y: {}", x, y);
        }

        self.data[(x + y * self.width) as usize] = color;

        self
    }

    pub fn flip_vertically(&mut self) -> &mut Self {
        let half = self.height >> 1;

        for y in 0..half {
            for x in 0..self.width {
                let top_line = self.coord(x, y);
                let bottom_line = self.coord(x, self.height - y - 1);

                self.data.swap(top_line, bottom_line);
            }
        }

        self
    }

    pub fn flip_horizontally(&mut self) -> &mut Self {
        let half = self.width >> 1;

        for x in 0..half {
            for y in 0..self.height {
                let left_line = self.coord(x, y);
                let right_line = self.coord(self.width - x - 1, y);

                self.data.swap(left_line, right_line);
            }
        }

        self
    }

    pub fn write_png<W: Write>(&self, w: W) -> Result<(), Error> {
        let mut encoder = Encoder::new(w, self.width, self.height);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data: Vec<_> = self
            .data
            .iter()
            .flat_map(|color| color.0.iter())
            .cloned()
            .collect();
        writer.write_image_data(&data)?;
        Ok(())
    }
}
