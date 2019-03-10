#![feature(test)]

use {
    nalgebra::{Point2, Vector2, Vector3},
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

fn ccw(a: Point2<isize>, b: Point2<isize>, c: Point2<isize>) -> isize {
    (b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])
}

fn is_in_triangle(pts: [Point2<isize>; 3], p: Point2<isize>) -> bool {
    match (
        ccw(pts[0], p, pts[1]) > 0,
        ccw(pts[1], p, pts[2]) > 0,
        ccw(pts[2], p, pts[0]) > 0,
    ) {
        (true, true, true) => true,
        (false, false, false) => true,
        _ => false,
    }
}

pub fn triangle(pts: [Point2<isize>; 3], image: &mut Image, color: Color) {
    let bbox_min_x = isize::min(isize::min(pts[0][0], pts[1][0]), pts[2][0]);
    let bbox_min_y = isize::min(isize::min(pts[0][1], pts[1][1]), pts[2][1]);
    let bbox_max_x = isize::max(isize::max(pts[0][0], pts[1][0]), pts[2][0]);
    let bbox_max_y = isize::max(isize::max(pts[0][1], pts[1][1]), pts[2][1]);

    for x in bbox_min_x..bbox_max_x {
        for y in bbox_min_y..bbox_max_y {
            let p = Point2::new(x, y);

            if is_in_triangle(pts, p) {
                image.set(p[0] as u32, p[1] as u32, color);
            }
        }
    }
}

fn world_to_screen_coords(p: Vector2<f32>, width: u32, height: u32) -> Vector2<f32> {
    Vector2::new(
        (p[0] + 1f32) * width as f32 / 2f32,
        (p[1] + 1f32) * height as f32 / 2f32,
    )
}

pub fn render_wireframe(model: &Obj, image: &mut Image, color: Color) {
    for face in model.indices.chunks_exact(3) {
        for &(i, j) in &[(0, 1), (1, 2), (2, 0)] {
            let v0: Vector3<_> = model.vertices[face[i] as usize].position.into();
            let v1: Vector3<_> = model.vertices[face[j] as usize].position.into();

            let (v0, v1) = (
                world_to_screen_coords(v0.xy(), image.width, image.height).map(|f| f as isize),
                world_to_screen_coords(v1.xy(), image.width, image.height).map(|f| f as isize),
            );

            line(v0.into(), v1.into(), image, color);
        }
    }
}

pub fn render_flat_shading(model: &Obj, image: &mut Image, color: Color, light_dir: Vector3<f32>) {
    for face in model.indices.chunks_exact(3) {
        let (screen_coords, world_coords): (Vec<Point2<_>>, Vec<Vector3<_>>) = face
            .iter()
            .map(|i| model.vertices[*i as usize].position.into())
            .map(|v: Vector3<_>| -> (Point2<_>, Vector3<_>) {
                (
                    world_to_screen_coords(v.xy(), image.width, image.height)
                        .map(|f| f as isize)
                        .into(),
                    v,
                )
            })
            .unzip();

        let n = (world_coords[2] - world_coords[0])
            .cross(&(world_coords[1] - world_coords[0]))
            .normalize();
        let intensity = n.dot(&light_dir);

        if intensity > 0f32 {
            let color = color * intensity;
            triangle(
                [screen_coords[0], screen_coords[1], screen_coords[2]],
                image,
                color,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use {
        crate::{line, triangle, Color, Image},
        nalgebra::Point2,
        test::Bencher,
    };

    const RED: Color = Color::from_rgba(255, 0, 0, 255);
    const WHITE: Color = Color::from_rgba(255, 255, 255, 255);
    const BLUE: Color = Color::from_rgba(0, 0, 255, 255);

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

    #[bench]
    fn bench_triangle(b: &mut Bencher) {
        let mut image = Image::new(200, 200);

        let t0 = [
            Point2::new(10, 70),
            Point2::new(50, 160),
            Point2::new(70, 80),
        ];
        let t1 = [
            Point2::new(180, 50),
            Point2::new(150, 1),
            Point2::new(70, 180),
        ];
        let t2 = [
            Point2::new(180, 150),
            Point2::new(120, 160),
            Point2::new(130, 180),
        ];

        b.iter(|| {
            triangle([t0[0], t0[1], t0[2]], &mut image, RED);
            triangle([t1[0], t1[1], t1[2]], &mut image, WHITE);
            triangle([t2[0], t2[1], t2[2]], &mut image, BLUE);
        });
    }
}
