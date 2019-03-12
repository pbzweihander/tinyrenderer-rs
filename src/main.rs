use {
    nalgebra::Vector3,
    obj::{load_obj, Obj, TexturedVertex},
    png::Decoder,
    std::io::BufReader,
    std::{env::args, fs::File},
    tinyrenderer::*,
};

const BLACK: Color = Color::from_rgba(0, 0, 0, 255);
// const WHITE: Color = Color::from_rgba(255, 255, 255, 255);
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
    let model: Obj<TexturedVertex> = load_obj(obj_file_buf_reader)?;

    let texture_file_name = args
        .next()
        .unwrap_or_else(|| "obj/african_head_diffuse.png".to_string());
    let texture_file = File::open(&texture_file_name)?;
    let texture_decoder = Decoder::new(texture_file);
    let (texture_info, mut texture_reader) = texture_decoder.read_info()?;
    let mut buf = vec![0; texture_info.buffer_size()];
    texture_reader.next_frame(&mut buf)?;

    let mut texture = Image {
        width: texture_info.width,
        height: texture_info.height,
        data: match texture_info.color_type {
            png::ColorType::RGB => buf
                .chunks_exact(3)
                .map(|c| Color::from_rgba(c[0], c[1], c[2], 255))
                .collect(),
            png::ColorType::RGBA => buf
                .chunks_exact(4)
                .map(|c| Color::from_rgba(c[0], c[1], c[2], c[3]))
                .collect(),
            _ => unimplemented!(),
        },
    };
    texture.flip_vertically();

    let out_file_name = args.next().unwrap_or_else(|| "output.png".to_string());
    let out_file = File::create(&out_file_name)?;

    let mut image = Image::with_background(WIDTH, HEIGHT, BLACK);
    render_flat_shading_with_texture(
        &model,
        &mut image,
        &texture,
        Vector3::new(0f32, 0f32, -1f32),
    );

    image.flip_vertically().write_png(out_file)?;

    Ok(())
}
