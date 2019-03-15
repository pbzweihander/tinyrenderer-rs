use {
    nalgebra::{Matrix3, Matrix4, Vector, Vector2, Vector3, Vector4},
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
