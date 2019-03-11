use {
    crate::{coord_to_idx, Error},
    png::{Encoder, HasParameters},
    std::{
        fmt,
        io::Write,
        ops::{Index, Mul},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color(pub [u8; 4]);

impl Color {
    pub const fn new() -> Self {
        Color([0, 0, 0, 0])
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color([r, g, b, a])
    }

    pub const fn r(self) -> u8 {
        self.0[0]
    }

    pub const fn g(self) -> u8 {
        self.0[1]
    }

    pub const fn b(self) -> u8 {
        self.0[2]
    }

    pub const fn a(self) -> u8 {
        self.0[3]
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.r(),
            self.g(),
            self.b(),
            self.a()
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new()
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Color([
            f32::max(
                f32::min(f32::from(self.r()) * rhs, f32::from(u8::max_value())),
                f32::from(u8::min_value()),
            ) as u8,
            f32::max(
                f32::min(f32::from(self.g()) * rhs, f32::from(u8::max_value())),
                f32::from(u8::min_value()),
            ) as u8,
            f32::max(
                f32::min(f32::from(self.b()) * rhs, f32::from(u8::max_value())),
                f32::from(u8::min_value()),
            ) as u8,
            self.a(),
        ])
    }
}

impl Index<usize> for Color {
    type Output = u8;

    fn index(&self, idx: usize) -> &u8 {
        &self.0[idx]
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            data: vec![Color::new(); (width * height) as usize],
        }
    }

    pub fn with_background(width: u32, height: u32, color: Color) -> Self {
        Image {
            width,
            height,
            data: vec![color; (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: Color) -> &mut Self {
        if x < self.width && y < self.height {
            self.data[coord_to_idx(x, y, self.width)] = color;
        }
        self
    }

    pub fn flip_vertically(&mut self) -> &mut Self {
        let half = self.height >> 1;

        for y in 0..half {
            let top_left = coord_to_idx(0, y, self.width);
            let bottom_right = coord_to_idx(self.width, self.height - y - 1, self.width);

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
                let left_line = coord_to_idx(x, y, self.width);
                let right_line = coord_to_idx(self.width - x - 1, y, self.width);

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
        let mut data: Vec<Vec<_>> = image.data.clone().chunks(255).map(|s| s.to_vec()).collect();

        image.flip_vertically();
        data.reverse();

        assert_eq!(
            image.data,
            data.clone()
                .into_iter()
                .flat_map(IntoIterator::into_iter)
                .collect::<Vec<_>>(),
        );
    }

    #[bench]
    fn bench_flip_vertically(b: &mut Bencher) {
        let mut image = make_image();

        b.iter(|| {
            image.flip_vertically();
        });
    }
}
