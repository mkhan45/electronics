use macroquad::miniquad::graphics::Context;
use macroquad::texture::Texture2D;
use resvg;
use usvg;

pub async fn texture_from_file(
    filename: &str,
    width: u32,
    height: u32,
    ctx: &mut Context,
) -> Texture2D {
    let bytes = macroquad::file::load_file(filename).await.unwrap();
    texture_from_bytes(bytes.as_slice(), width, height, ctx)
}

pub fn texture_from_bytes(bytes: &[u8], width: u32, height: u32, ctx: &mut Context) -> Texture2D {
    let svg_tree = usvg::Tree::from_data(bytes, &usvg::Options::default()).unwrap();

    let pixmap = {
        let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
        resvg::render(&svg_tree, usvg::FitTo::Original, pixmap.as_mut());
        pixmap
    };

    Texture2D::from_rgba8(ctx, width as u16, height as u16, pixmap.data())
}
