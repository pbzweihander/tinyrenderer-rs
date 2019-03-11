use {
    nalgebra::Vector3,
    obj::{load_obj, Obj},
    std::io::BufReader,
    std::{env::args, fs::File},
    tinyrenderer::*,
};

const BLACK: Color = Color::from_rgba(0, 0, 0, 255);
const WHITE: Color = Color::from_rgba(255, 255, 255, 255);
// const RED: Color = Color::from_rgba(255, 0, 0, 255);
// const GREEN: Color = Color::from_rgba(0, 255, 0, 255);
// const BLUE: Color = Color::from_rgba(0, 0, 255, 255);

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() -> Result<(), Error> {
    let mut args = args();
    args.next();

    let obj_file_name = args
        .next()
        .unwrap_or_else(|| "obj/african_head.obj".to_string());
    let obj_file = File::open(&obj_file_name)?;
    let obj_file_buf_reader = BufReader::new(obj_file);
    let model: Obj = load_obj(obj_file_buf_reader)?;

    let out_file_name = args.next().unwrap_or_else(|| "output.png".to_string());
    let out_file = File::create(&out_file_name)?;

    let mut image = Image::with_background(WIDTH, HEIGHT, BLACK);
    render_flat_shading(&model, &mut image, WHITE, Vector3::new(0f32, 0f32, -1f32));

    image.flip_vertically().write_png(out_file)?;

    Ok(())
}
