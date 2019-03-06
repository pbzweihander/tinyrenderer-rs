#![feature(test)]

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

    #[inline]
    const fn coord(&self, x: u32, y: u32) -> usize {
        (self.width * y + x) as usize
    }

    pub fn set(&mut self, x: u32, y: u32, color: Color) -> &mut Self {
        if x >= self.width || y >= self.height {
            panic!("invalid coordinate x: {}, y: {}", x, y);
        }

        self.data[(x + y * self.width) as usize] = color;

        self
    }

    #[cfg(test)]
    fn flip_vertically_orig(&mut self) -> &mut Self {
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

    pub fn flip_vertically(&mut self) -> &mut Self {
        let half = self.height >> 1;

        for y in 0..half {
            let top_left = self.coord(0, y);
            let bottom_right = self.coord(self.width, self.height - y - 1);

            let s = &mut self.data[top_left..bottom_right];
            let (top_line, s) = s.split_at_mut(self.width as usize);
            let (_, bottom_line) = s.split_at_mut(s.len() - self.width as usize);

            top_line.swap_with_slice(bottom_line);
        }

        self
    }

    pub fn flip_horizontally(&mut self) -> &mut Self {
        let half = self.width >> 1;

        for y in 0..self.height {
            for x in 0..half {
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

#[cfg(test)]
mod tests {
    extern crate test;

    use {super::*, test::Bencher};

    fn make_image() -> Image {
        let mut image = Image::new(255, 255);

        for y in 0..255 {
            for x in 0..255 {
                image.set(
                    x,
                    y,
                    Color::from_rgba((x) as u8, (255 - x) as u8, (y) as u8, (255 - y) as u8),
                );
            }
        }

        image
    }

    #[test]
    fn test_flip_vertically() {
        let mut image = make_image();
        let mut image1 = image.clone();
        let mut data: Vec<Vec<_>> = image.data.clone().chunks(255).map(|s| s.to_vec()).collect();

        image.flip_vertically();
        image1.flip_vertically_orig();
        data.reverse();

        assert_eq!(
            image.data,
            data.clone()
                .into_iter()
                .flat_map(IntoIterator::into_iter)
                .collect::<Vec<_>>(),
        );
        assert_eq!(image.data, image1.data);
    }

    #[bench]
    fn bench_flip_vertically(b: &mut Bencher) {
        let mut image = make_image();

        b.iter(|| {
            image.flip_vertically();
        });
    }

    #[bench]
    fn bench_flip_vertically_orig(b: &mut Bencher) {
        let mut image = make_image();

        b.iter(|| {
            image.flip_vertically_orig();
        });
    }
}
