#![feature(test)]

use obj::Obj;

pub use failure::Error;

pub mod image;
pub mod vec;

pub use crate::{
    image::{Color, Image},
    vec::{Vec2, Vec3},
};

pub fn line(v0: Vec2, v1: Vec2, image: &mut Image, color: Color) {
    let mut steep = false;

    let (Vec2(x0, y0), Vec2(x1, y1)) = (v0, v1);

    let (x0, x1, y0, y1) = if isize::abs(x0 - x1) < isize::abs(y0 - y1) {
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

    let dx = x1 - x0;
    let dy = y1 - y0;

    let derror2 = isize::abs(dy) * 2;
    let mut error2 = 0;

    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            image.set(y as u32, x as u32, color);
        } else {
            image.set(x as u32, y as u32, color);
        }

        error2 += derror2;
        if error2 > dx {
            y = if y1 > y0 { y + 1 } else { y - 1 };
            error2 -= dx * 2;
        }
    }
}

pub fn render_wireframe(model: Obj, image: &mut Image, color: Color) {
    for face in model.indices.chunks_exact(3) {
        for &(i, j) in &[(0, 1), (1, 2), (2, 0)] {
            let v0: Vec3<_> = model.vertices[face[i] as usize].position.into();
            let v1: Vec3<_> = model.vertices[face[j] as usize].position.into();

            let (v0, v1): (Vec2<f32>, Vec2<f32>) = (v0.into(), v1.into());

            let (v0, v1) = (
                ((v0 + Vec2(1f32, 1f32)) / 2f32)
                    .hadamard(Vec2(image.width as f32, image.height as f32)),
                ((v1 + Vec2(1f32, 1f32)) / 2f32)
                    .hadamard(Vec2(image.width as f32, image.height as f32)),
            );

            let (v0, v1) = (v0.map(|f| f as isize), v1.map(|f| f as isize));

            line(v0, v1, image, color);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use {
        crate::{line, Color, Image, Vec2},
        test::Bencher,
    };

    const RED: Color = Color::from_rgba(255, 0, 0, 255);
    const WHITE: Color = Color::from_rgba(255, 255, 255, 255);

    #[bench]
    fn bench_line(b: &mut Bencher) {
        let mut image = Image::new(1000, 1000);

        let v0 = Vec2(130, 200);
        let v1 = Vec2(200, 130);
        let v2 = Vec2(800, 400);
        let v3 = Vec2(400, 800);

        b.iter(|| {
            line(v0, v2, &mut image, WHITE);
            line(v1, v3, &mut image, RED);
            line(v2, v0, &mut image, RED);
        });
    }
}
