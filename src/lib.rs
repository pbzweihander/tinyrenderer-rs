#![feature(test)]

use {
    nalgebra::{Vector, Vector2, Vector3},
    obj::{Obj, TexturedVertex},
};

pub use failure::Error;

pub mod image;

pub use crate::image::{Color, Image};

#[inline]
pub(crate) const fn coord_to_idx(x: u32, y: u32, width: u32) -> usize {
    (width * y + x) as usize
}

// #[inline]
// pub(crate) const fn idx_to_coord(idx: usize, width: u32) -> (u32, u32) {
//     ((idx % width as usize) as u32, (idx / width as usize) as u32)
// }

pub fn line(v0: Vector2<isize>, v1: Vector2<isize>, image: &mut Image, color: Color) {
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

fn barycentric(pts: [Vector2<f32>; 3], p: Vector2<f32>) -> Vector3<f32> {
    use nalgebra::Matrix2x3;

    let ab = pts[1] - pts[0];
    let ac = pts[2] - pts[0];
    let pa = pts[0] - p;

    let m = Matrix2x3::from_columns(&[ab, ac, pa]);

    let x = m.row(0);
    let y = m.row(1);

    let uv1 = x.cross(&y);

    if f32::abs(uv1[2]) > 0.01 {
        Vector3::new(
            1.0 - (uv1[0] + uv1[1]) / uv1[2],
            uv1[1] / uv1[2],
            uv1[0] / uv1[2],
        )
    } else {
        Vector3::new(-1.0, 1.0, 1.0)
    }
}

fn un_barycentric(pts: [Vector2<f32>; 3], p: Vector3<f32>) -> Vector2<f32> {
    use nalgebra::Matrix2x3;

    let m = Matrix2x3::from_columns(&[pts[0], pts[1], pts[2]]).transpose();
    let v = p.transpose() * m;

    Vector2::new(v[0], v[1])
}

pub fn triangle(pts: [Vector2<f32>; 3], image: &mut Image, color: Color) {
    let bbox_min_x = f32::min(f32::min(pts[0][0], pts[1][0]), pts[2][0]);
    let bbox_min_y = f32::min(f32::min(pts[0][1], pts[1][1]), pts[2][1]);
    let bbox_max_x = f32::max(f32::max(pts[0][0], pts[1][0]), pts[2][0]);
    let bbox_max_y = f32::max(f32::max(pts[0][1], pts[1][1]), pts[2][1]);

    for x in (bbox_min_x.floor() as u32)..(bbox_max_x.ceil() as u32) {
        for y in (bbox_min_y.floor() as u32)..(bbox_max_y.ceil() as u32) {
            let p = Vector2::new(x as f32, y as f32);

            let bc_screen = barycentric(pts, p);

            if bc_screen[0] >= 0.0 && bc_screen[1] >= 0.0 && bc_screen[2] >= 0.0 {
                image.set(x, y, color);
            }
        }
    }
}

pub fn triangle_with_zbuffer(
    pts: [Vector3<f32>; 3],
    zbuffer: &mut [f32],
    image: &mut Image,
    color: Color,
) {
    let bbox_min_x = f32::min(f32::min(pts[0][0], pts[1][0]), pts[2][0]);
    let bbox_min_y = f32::min(f32::min(pts[0][1], pts[1][1]), pts[2][1]);
    let bbox_max_x = f32::max(f32::max(pts[0][0], pts[1][0]), pts[2][0]);
    let bbox_max_y = f32::max(f32::max(pts[0][1], pts[1][1]), pts[2][1]);

    for x in (bbox_min_x.floor() as u32)..(bbox_max_x.ceil() as u32) {
        for y in (bbox_min_y.floor() as u32)..(bbox_max_y.ceil() as u32) {
            let p = Vector2::new(x as f32, y as f32);

            let bc_screen = barycentric([pts[0].xy(), pts[1].xy(), pts[2].xy()], p);

            if bc_screen[0] >= 0.0 && bc_screen[1] >= 0.0 && bc_screen[2] >= 0.0 {
                let mut z = 0.0;

                for i in 0..3 {
                    z += pts[i][2] * bc_screen[i];
                }

                let idx = coord_to_idx(x, y, image.width);

                if zbuffer[idx] < z {
                    zbuffer[idx] = z;
                    image.data[idx] = color;
                }
            }
        }
    }
}

pub fn triangle_with_zbuffer_with_texture(
    pts: [Vector3<f32>; 3],
    zbuffer: &mut [f32],
    image: &mut Image,
    texture: &Image,
    texture_coords: [Vector3<f32>; 3],
    intensity: f32,
) {
    let bbox_min_x = f32::min(f32::min(pts[0][0], pts[1][0]), pts[2][0]);
    let bbox_min_y = f32::min(f32::min(pts[0][1], pts[1][1]), pts[2][1]);
    let bbox_max_x = f32::max(f32::max(pts[0][0], pts[1][0]), pts[2][0]);
    let bbox_max_y = f32::max(f32::max(pts[0][1], pts[1][1]), pts[2][1]);

    for x in (bbox_min_x.floor() as u32)..(bbox_max_x.ceil() as u32) {
        for y in (bbox_min_y.floor() as u32)..(bbox_max_y.ceil() as u32) {
            let p = Vector2::new(x as f32, y as f32);

            let bc_screen = barycentric([pts[0].xy(), pts[1].xy(), pts[2].xy()], p);

            if bc_screen[0] >= 0.0 && bc_screen[1] >= 0.0 && bc_screen[2] >= 0.0 {
                let mut z = 0.0;

                for i in 0..3 {
                    z += pts[i][2] * bc_screen[i];
                }

                let idx = coord_to_idx(x, y, image.width);

                if zbuffer[idx] < z {
                    let texture_coords = texture_coords
                        .iter()
                        .map(|vt| {
                            Vector2::new(
                                vt[0] * texture.width as f32,
                                vt[1] * texture.height as f32,
                            )
                        })
                        .collect::<Vec<_>>();

                    let vt = un_barycentric(
                        [texture_coords[0], texture_coords[2], texture_coords[1]],
                        bc_screen,
                    );

                    let t_idx = coord_to_idx(vt[0] as u32, vt[1] as u32, texture.width);

                    let color = texture.data[t_idx] * intensity;

                    zbuffer[idx] = z;
                    image.data[idx] = color;
                }
            }
        }
    }
}

fn world_to_screen_coords<D, S>(
    mut p: Vector<f32, D, S>,
    width: u32,
    height: u32,
) -> Vector<f32, D, S>
where
    D: nalgebra::Dim,
    S: nalgebra::base::storage::StorageMut<f32, D>,
{
    p[0] = (p[0] + 1.0) * width as f32 / 2.0;
    p[1] = (p[1] + 1.0) * height as f32 / 2.0;

    p
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

            line(v0, v1, image, color);
        }
    }
}

pub fn render_flat_shading(model: &Obj, image: &mut Image, color: Color, light_dir: Vector3<f32>) {
    let mut zbuffer = vec![std::f32::MIN; image.height as usize * image.width as usize];

    for face in model.indices.chunks_exact(3) {
        let (screen_coords, world_coords): (Vec<Vector3<_>>, Vec<Vector3<_>>) = face
            .iter()
            .map(|i| model.vertices[*i as usize].position.into())
            .map(|v| (world_to_screen_coords(v, image.width, image.height), v))
            .unzip();

        let n = (world_coords[2] - world_coords[0])
            .cross(&(world_coords[1] - world_coords[0]))
            .normalize();
        let intensity = n.dot(&light_dir);

        if intensity > 0.0 {
            let color = color * intensity;
            triangle_with_zbuffer(
                [screen_coords[0], screen_coords[1], screen_coords[2]],
                &mut zbuffer,
                image,
                color,
            );
        }
    }
}

pub fn render_flat_shading_with_texture(
    model: &Obj<TexturedVertex>,
    image: &mut Image,
    texture: &Image,
    light_dir: Vector3<f32>,
) {
    let mut zbuffer = vec![std::f32::MIN; image.height as usize * image.width as usize];

    for face in model.indices.chunks_exact(3) {
        let (mut screen_coords, mut world_coords, mut texture_coords): (
            Vec<Vector3<_>>,
            Vec<Vector3<_>>,
            Vec<Vector3<_>>,
        ) = (
            Vec::with_capacity(3),
            Vec::with_capacity(3),
            Vec::with_capacity(3),
        );

        for &i in face {
            let v = model.vertices[i as usize];
            let vp: Vector3<_> = v.position.into();

            screen_coords.push(world_to_screen_coords(vp, image.width, image.height));
            world_coords.push(vp);
            texture_coords.push(v.texture.into());
        }

        let (screen_coords, world_coords, texture_coords) =
            (screen_coords, world_coords, texture_coords);

        let n = (world_coords[2] - world_coords[0])
            .cross(&(world_coords[1] - world_coords[0]))
            .normalize();
        let intensity = n.dot(&light_dir);

        if intensity > 0.0 {
            triangle_with_zbuffer_with_texture(
                [screen_coords[0], screen_coords[1], screen_coords[2]],
                &mut zbuffer,
                image,
                texture,
                [texture_coords[0], texture_coords[1], texture_coords[2]],
                intensity,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use {
        crate::{line, triangle, triangle_with_zbuffer, Color, Image},
        nalgebra::{Vector2, Vector3},
        test::Bencher,
    };

    const WHITE: Color = Color::from_rgba(255, 255, 255, 255);
    const RED: Color = Color::from_rgba(255, 0, 0, 255);
    const GREEN: Color = Color::from_rgba(0, 255, 0, 255);
    const BLUE: Color = Color::from_rgba(0, 0, 255, 255);

    #[bench]
    fn bench_line(b: &mut Bencher) {
        let mut image = Image::new(1000, 1000);

        let v0 = Vector2::new(130, 200);
        let v1 = Vector2::new(200, 130);
        let v2 = Vector2::new(800, 400);
        let v3 = Vector2::new(400, 800);

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
            Vector2::new(10.0, 70.0),
            Vector2::new(50.0, 160.0),
            Vector2::new(70.0, 80.0),
        ];
        let t1 = [
            Vector2::new(180.0, 50.0),
            Vector2::new(150.0, 1.0),
            Vector2::new(70.0, 180.0),
        ];
        let t2 = [
            Vector2::new(180.0, 150.0),
            Vector2::new(120.0, 160.0),
            Vector2::new(130.0, 180.0),
        ];

        b.iter(|| {
            triangle(t0, &mut image, RED);
            triangle(t1, &mut image, WHITE);
            triangle(t2, &mut image, BLUE);
        });
    }

    #[bench]
    fn bench_triangle_with_zbuffer(b: &mut Bencher) {
        let mut image = Image::new(800, 800);
        let mut zbuffer = vec![std::f32::MIN; 800 * 800];

        let t0 = [
            Vector3::new(20.0, 400.0, 34.0),
            Vector3::new(744.0, 600.0, 400.0),
            Vector3::new(744.0, 200.0, 400.0),
        ];
        let t1 = [
            Vector3::new(120.0, 700.0, 434.0),
            Vector3::new(120.0, 100.0, 434.0),
            Vector3::new(444.0, 400.0, 400.0),
        ];
        let t2 = [
            Vector3::new(330.0, 400.0, 463.0),
            Vector3::new(594.0, 5.0, 200.0),
            Vector3::new(594.0, 795.0, 200.0),
        ];

        b.iter(|| {
            triangle_with_zbuffer(t0, &mut zbuffer, &mut image, RED);
            triangle_with_zbuffer(t1, &mut zbuffer, &mut image, GREEN);
            triangle_with_zbuffer(t2, &mut zbuffer, &mut image, BLUE);
        });
    }
}
