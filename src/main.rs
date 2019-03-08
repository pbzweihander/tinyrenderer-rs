use {
    obj::{load_obj, Obj},
    std::{env::args, fs::File, io::BufReader},
    tinyrenderer::*,
};

const BLACK: Color = Color::from_rgba(0, 0, 0, 255);
const WHITE: Color = Color::from_rgba(255, 255, 255, 255);

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

    let mut image = Image::with_background(800, 800, BLACK);

    render_wireframe(model, &mut image, WHITE);
    image.flip_vertically().write_png(out_file)?;

    Ok(())
}
