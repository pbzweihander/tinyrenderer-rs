use {
    std::{env::args, fs::File, path::Path},
    tinyrenderer::{Color, Error, Image},
};

const RED: Color = Color::from_rgba(255, 0, 0, 255);

fn main() -> Result<(), Error> {
    let mut args = args();
    args.next();
    let filename = args.next().unwrap_or_else(|| "output.png".to_string());
    let filepath = Path::new(&filename);
    let file = File::create(filepath)?;

    let mut image = Image::new(100, 100);
    image.set(52, 41, RED).flip_vertically().write_png(file)?;

    Ok(())
}
