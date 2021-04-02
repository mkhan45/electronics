use macroquad::texture::Texture2D;

#[derive(Default)]
pub struct TickProgress(pub f64);

#[derive(Default)]
pub struct Textures(pub std::collections::BTreeMap<String, Texture2D>);
