/*
 pub mod wall
 Made by: Mujibullah Muhebullah
 March 27 2026
*/
use crate::modules::collision::check_collision;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub struct Wall {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
}

impl Wall {
    pub async fn new(asset_path: &str, 
        width: f32, 
        height: f32, 
        x: f32, 
        y: f32, 
        stretch_enabled: bool, 
        zoom_level: f32, 
        ) -> Self {
        Wall {
            view: StillImage::new(&asset_path, 50.0, 50.0, x, y, true, 1.0).await,
            move_speed: 200.0,
            movement: Vec2::ZERO,
        }
    }
    pub fn position(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }
    pub fn get_x(&self) -> f32 {
        self.view.get_x()
    }
    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }
    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
    }
    pub fn set_y(&mut self, y: f32) {
        self.view.set_y(y);
    }
    fn set_position(&mut self, x: f32, y: f32) {
        self.set_x(x);
        self.set_y(y);
    }
    pub fn is_moving() {
       
    }
    pub fn draw(&mut self) {
        self.view.draw();
    }
}
