#![feature(test)]

use {
    std::{env::args, fs::File, path::Path},
    tinyrenderer::{Color, Error, Image},
};

const RED: Color = Color::from_rgba(255, 0, 0, 255);
const WHITE: Color = Color::from_rgba(255, 255, 255, 255);

fn diff(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn line(x0: u32, y0: u32, x1: u32, y1: u32, image: &mut Image, color: Color) {
    let mut steep = false;

    let (x0, x1, y0, y1) = if diff(x0, x1) < diff(y0, y1) {
        steep = true;
        (y0, y1, x0, x1)
    } else {
        (x0, x1, y0, y1)
    };

    let (x0, x1, y0, y1) = if x0 > x1 {
        (x1, x0, y1, y0)
    } else {
        (x0, x1, y0, y1)
    };

    let dx = x1 as isize - x0 as isize;
    let dy = y1 as isize - y0 as isize;

    let derror2 = isize::abs(dy) * 2;
    let mut error2 = 0;

    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            image.set(y, x, color);
        } else {
            image.set(x, y, color);
        }
        error2 += derror2;
        if error2 > dx {
            y = if y1 > y0 { y + 1 } else { y - 1 };
            error2 -= dx * 2;
        }
    }
}

fn main() -> Result<(), Error> {
    let mut args = args();
    args.next();
    let filename = args.next().unwrap_or_else(|| "output.png".to_string());
    let filepath = Path::new(&filename);
    let file = File::create(filepath)?;

    let mut image = Image::new(100, 100);
    line(13, 20, 80, 40, &mut image, WHITE);
    line(20, 13, 40, 80, &mut image, RED);
    line(80, 40, 13, 20, &mut image, RED);
    image.flip_vertically().write_png(file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate test;

    use {
        super::{line, Image, RED, WHITE},
        test::Bencher,
    };

    #[bench]
    fn bench_line(b: &mut Bencher) {
        let mut image = Image::new(1000, 1000);

        b.iter(|| {
            line(130, 200, 800, 400, &mut image, WHITE);
            line(200, 130, 400, 800, &mut image, RED);
            line(800, 400, 130, 200, &mut image, RED);
        });
    }
}
