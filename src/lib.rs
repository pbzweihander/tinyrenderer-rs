#![feature(test)]

use {
    nalgebra::{Point2, Point3, Vector2, Vector3},
    obj::Obj,
};

pub use failure::Error;

pub mod image;

pub use crate::image::{Color, Image};

pub fn line(v0: Point2<isize>, v1: Point2<isize>, image: &mut Image, color: Color) {
    let mut steep = false;

    let (x0, y0, x1, y1) = (v0[0], v0[1], v1[0], v1[1]);

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
            let v0: Vector3<_> = model.vertices[face[i] as usize].position.into();
            let v1: Vector3<_> = model.vertices[face[j] as usize].position.into();

            let (v0, v1) = (v0.xy(), v1.xy());

            let (mut v0, mut v1) = (
                ((v0 + Vector2::new(1f32, 1f32)) / 2f32),
                ((v1 + Vector2::new(1f32, 1f32)) / 2f32),
            );
            v0[0] *= image.width as f32;
            v1[0] *= image.width as f32;
            v0[1] *= image.height as f32;
            v1[1] *= image.height as f32;

            let (v0, v1) = (v0.map(|f| f as isize), v1.map(|f| f as isize));

            let (p0, p1): (Point2<_>, Point2<_>) = (v0.into(), v1.into());

            line(p0, p1, image, color);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use {
        crate::{line, Color, Image},
        nalgebra::Point2,
        test::Bencher,
    };

    const RED: Color = Color::from_rgba(255, 0, 0, 255);
    const WHITE: Color = Color::from_rgba(255, 255, 255, 255);

    #[bench]
    fn bench_line(b: &mut Bencher) {
        let mut image = Image::new(1000, 1000);

        let v0 = Point2::new(130, 200);
        let v1 = Point2::new(200, 130);
        let v2 = Point2::new(800, 400);
        let v3 = Point2::new(400, 800);

        b.iter(|| {
            line(v0, v2, &mut image, WHITE);
            line(v1, v3, &mut image, RED);
            line(v2, v0, &mut image, RED);
        });
    }
}
