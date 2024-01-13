use macroquad::prelude::*;
use rand;

#[macroquad::main("Auto moon!")]
async fn main() {
    let mut camera = Camera {
        position: vec2(0.0, 0.0),
        zoom: 1.0,
    };
    let tile_map = TileMap::new().await;

    loop {
        clear_background(BLACK);

        camera.update();
        tile_map.draw(&camera);
        tile_map.draw_hover_frame(&camera);

        next_frame().await
    }
}

struct Camera {
    position: Vec2,
    zoom: f32,
}
const CAMERA_SPEED: f32 = 4.0;
const CAMERA_ZOOM_SPEED: f32 = 0.001;
impl Camera {
    fn update(&mut self) {
        // controls
        if is_key_down(KeyCode::W) {
            self.position.y += CAMERA_SPEED * self.zoom;
        }
        if is_key_down(KeyCode::S) {
            self.position.y -= CAMERA_SPEED * self.zoom;
        }
        if is_key_down(KeyCode::A) {
            self.position.x += CAMERA_SPEED * self.zoom;
        }
        if is_key_down(KeyCode::D) {
            self.position.x -= CAMERA_SPEED * self.zoom;
        }
        // zoom mouse wheel
        let zoom_delta = mouse_wheel().1;
        if zoom_delta != 0.0 {
            self.zoom += zoom_delta * self.zoom * CAMERA_ZOOM_SPEED;
        }
    }
}

#[derive(Clone)]
struct Construction {
    texture: Texture2D,
}


const TEXTURE_SIZE: f32 = 18.0;
#[derive(Clone)]
struct Tile {
    texture: Texture2D,
    constructions: Vec<Construction>,
}

const TILE_MAP_DIMENSION: (usize, usize) = (10, 10) ;
struct TileMap {
    tiles: Vec<Vec<Tile>>,
}
impl TileMap {
    async fn new() -> Self {
        let mut tile_map: TileMap = TileMap { tiles: Vec::with_capacity(TILE_MAP_DIMENSION.0) };

        for i in 0..TILE_MAP_DIMENSION.0 {
            let mut linha = Vec::with_capacity(TILE_MAP_DIMENSION.1);
            for j in 0..TILE_MAP_DIMENSION.1 {
                let tile = Tile {
                    texture: load_texture("assets/tiles/terra.png").await.unwrap(),
                    constructions: Vec::new(),
                };
                linha.push(tile);
            }
            tile_map.tiles.push(linha);
        }

        tile_map
    }
    fn draw(&self, camera: &Camera) {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                let tile = &self.tiles[i][j];
                draw_texture_ex(
                    &tile.texture,
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
    fn screen2tile(&self, camera: &Camera, position: Vec2) -> Vec2 {
        let x = (position.x - camera.position.x) / (TEXTURE_SIZE * camera.zoom);
        let y = (position.y - camera.position.y) / (TEXTURE_SIZE * camera.zoom);

        vec2(x, y)
    }
    fn get_tile(&self, position: Vec2) -> Option<&Tile> {
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
    fn get_tile_mut(&mut self, position: Vec2) -> Option<&mut Tile> {
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
    fn draw_hover_frame(&self, camera: &Camera) {
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
                2.0,
                RED,
            );
        }
    }

}