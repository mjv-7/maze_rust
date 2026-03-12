use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct Wall {
    pub texture: Texture2D,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Wall {
    pub fn new(texture: Texture2D, x: f32, y: f32, width: f32, height: f32) -> Self {
        Wall {
            texture,
            x,
            y,
            width,
            height,
        }
    }
}
