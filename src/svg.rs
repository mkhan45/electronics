use macroquad::miniquad::graphics::Context;
use macroquad::texture::Texture2D;
use resvg;
use std::path::Path;
use usvg;

pub fn texture_from_file(
    filename: impl AsRef<Path>,
    width: u32,
    height: u32,
    ctx: &mut Context,
) -> Texture2D {
    let svg_tree = usvg::Tree::from_file(filename, &usvg::Options::default()).unwrap();

    let pixmap = {
        let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
        resvg::render(&svg_tree, usvg::FitTo::Original, pixmap.as_mut());
        pixmap.to_owned()
    };

    Texture2D::from_rgba8(ctx, width as u16, height as u16, pixmap.data())
}
