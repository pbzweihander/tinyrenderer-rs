use {
    nalgebra::Point2,
    // obj::{load_obj, Obj},
    // std::io::BufReader,
    std::{env::args, fs::File},
    tinyrenderer::*,
};

const BLACK: Color = Color::from_rgba(0, 0, 0, 255);
const WHITE: Color = Color::from_rgba(255, 255, 255, 255);
const RED: Color = Color::from_rgba(255, 0, 0, 255);
const BLUE: Color = Color::from_rgba(0, 0, 255, 255);

fn main() -> Result<(), Error> {
    let mut args = args();
    args.next();

    // let obj_file_name = args
    //     .next()
    //     .unwrap_or_else(|| "obj/african_head.obj".to_string());
    // let obj_file = File::open(&obj_file_name)?;
    // let obj_file_buf_reader = BufReader::new(obj_file);
    // let model: Obj = load_obj(obj_file_buf_reader)?;

    let out_file_name = args.next().unwrap_or_else(|| "output.png".to_string());
    let out_file = File::create(&out_file_name)?;

    // let mut image = Image::with_background(800, 800, BLACK);
    // render_wireframe(model, &mut image, WHITE);

    let mut image = Image::with_background(200, 200, BLACK);

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

    triangle(t0[0], t0[1], t0[2], &mut image, RED);
    triangle(t1[0], t1[1], t1[2], &mut image, WHITE);
    triangle(t2[0], t2[1], t2[2], &mut image, BLUE);

    image.flip_vertically().write_png(out_file)?;

    Ok(())
}
