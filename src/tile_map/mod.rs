use macroquad::color::{RED, WHITE};
use macroquad::input::mouse_position;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_rectangle_lines, draw_texture_ex, DrawTextureParams};

pub(crate) mod tile;
use crate::tile_map::tile::{Tile, TEXTURE_SIZE, TileTexture};
use crate::camera::Camera;

pub const TILE_MAP_DIMENSION: (usize, usize) = (100, 100) ;

pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
}
impl TileMap {
    pub async fn new() -> Self {
        let mut tile_map: TileMap = TileMap { tiles: Vec::with_capacity(TILE_MAP_DIMENSION.0) };

        for i in 0..TILE_MAP_DIMENSION.0 {
            let mut linha = Vec::with_capacity(TILE_MAP_DIMENSION.1);
            for j in 0..TILE_MAP_DIMENSION.1 {
                let tile = Tile {
                    texture: TileTexture::Grass.get(),
                    constructions: Vec::new(),
                };
                linha.push(tile);
            }
            tile_map.tiles.push(linha);
        }

        tile_map
    }
    pub fn draw(&self, camera: &Camera) {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                let tile = &self.tiles[i][j];
                draw_texture_ex(
                    tile.texture,
                    camera.position.x + i as f32 * TEXTURE_SIZE * camera.zoom,
                    camera.position.y + j as f32 * TEXTURE_SIZE * camera.zoom,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(TEXTURE_SIZE * camera.zoom, TEXTURE_SIZE * camera.zoom)),
                        ..Default::default()
                    },
                );
            }
        }
    }
    pub fn screen2tile(&self, camera: &Camera, position: Vec2) -> Vec2 {
        let x = (position.x - camera.position.x) / (TEXTURE_SIZE * camera.zoom);
        let y = (position.y - camera.position.y) / (TEXTURE_SIZE * camera.zoom);

        vec2(x, y)
    }
    pub fn get_tile(&self, position: Vec2) -> Option<&Tile> {
        if position.x < 0.0 || position.y < 0.0 {
            return None;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        if x < self.tiles.len() && y < self.tiles[x].len() {
            Some(&self.tiles[x][y])
        } else {
            None
        }
    }
    pub fn get_tile_mut(&mut self, position: Vec2) -> Option<&mut Tile> {
        if position.x < 0.0 || position.y < 0.0 {
            return None;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        if x < self.tiles.len() && y < self.tiles[x].len() {
            Some(&mut self.tiles[x][y])
        } else {
            None
        }
    }
    pub fn draw_hover_frame(&self, camera: &Camera) {
        let mouse_tuple = mouse_position();
        let mouse_position = vec2(mouse_tuple.0, mouse_tuple.1);

        let tile_position = self.screen2tile(camera, mouse_position);
        let tile = self.get_tile(tile_position);
        if let Some(tile) = tile {
            draw_rectangle_lines(
                camera.position.x + tile_position.x.trunc() * TEXTURE_SIZE * camera.zoom,
                camera.position.y + tile_position.y.trunc() * TEXTURE_SIZE * camera.zoom,
                TEXTURE_SIZE * camera.zoom,
                TEXTURE_SIZE * camera.zoom,
                2.0 * camera.zoom,
                RED,
            );
        }
    }

}