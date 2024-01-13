mod tile_map;
mod camera;

use macroquad::prelude::*;
use tile_map::*;
use tile_map::tile;

#[macroquad::main("Auto moon!")]
async fn main() {
    unsafe { tile::TileTexture::load().await; }

    let mut camera = camera::Camera::new();
    let tile_map = TileMap::new().await;

    loop {
        clear_background(BLACK);

        camera.update();
        tile_map.draw(&camera);
        tile_map.draw_hover_frame(&camera);

        next_frame().await
    }
}
