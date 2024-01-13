use macroquad::input::{is_key_down, KeyCode, mouse_wheel};
use macroquad::math::{Vec2, vec2};

const CAMERA_SPEED: f32 = 4.0;
const CAMERA_ZOOM_SPEED: f32 = 0.001;

pub(crate) struct Camera {
    pub(crate) position: Vec2,
    pub(crate) zoom: f32,
}
impl Camera {
    pub fn new() -> Self {
        Camera {
            position: vec2(0.0, 0.0),
            zoom: 1.0,
        }
    }
    pub fn update(&mut self) {
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
