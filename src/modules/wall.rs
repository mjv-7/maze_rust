use macroquad::prelude::*;
use crate::modules::still_image::{self, StillImage};


pub struct Wall {
    pub view: StillImage,
    pub move_speed: f32,
    movement: Vec2,
}

impl Wall {

    pub async fn new(asset_path: String, move_speed: f32,  width: f32, height: f32, x: f32, y: f32,) -> Self {
        Wall {
            view: StillImage::new(&asset_path, 50.0, 50.0, x, y, true, 1.0).await,
            move_speed: move_speed,
            movement: vec2(0.0, 0.0),
        }
    }
    pub fn moving(&mut self, img_out: &StillImage) -> Self {
        if img_out.collision() {

        }
            
    }
    pub fn draw(&mut self) {
        self.view.draw();
    }
}