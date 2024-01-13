use macroquad::prelude::{load_texture, Texture2D};
use macros::TileTextureLoadGet;

pub const TEXTURE_SIZE: f32 = 32.0;


#[derive(Clone, TileTextureLoadGet)]
pub enum TileTexture {
    Grass,
}

#[derive(Clone)]
pub struct Construction {
    pub texture: &'static Texture2D,
}

#[derive(Clone)]
pub struct Tile {
    pub texture: &'static Texture2D,
    pub constructions: Vec<Construction>,
}