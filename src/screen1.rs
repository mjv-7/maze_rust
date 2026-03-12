use macroquad::prelude::*;
use crate::modules::text_button::TextButton;
use crate::modules::grid::{draw_grid};
use crate::modules::label::Label;
use crate::modules::scale::use_virtual_resolution;


pub async fn run() -> String {
    use_virtual_resolution(1440.0, 1080.0);
    let mut lbl_out = Label::new("Beautiful Maze Game", 600.0, 50.0, 30);
    lbl_out.with_colors(WHITE, Some(BLACK));
     let btn_text = TextButton::new(
        400.0,
        200.0,
        200.0,
        60.0,
        "Click Me",
        BLACK,
        GREEN,
        30
     );
     let btn_exit = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Exit",
        BLACK,
        GREEN,
        30
     );
    loop {
    clear_background(BLUE);
    
    draw_text("By Mujibullah", 20.0, 40.0, 30.0, WHITE);
    lbl_out.draw();
    
    if btn_text.click() || is_key_down(KeyCode::Backspace) {
        return "screen2".to_string();
    }
    
    if btn_exit.click() {
        std::process::exit(0);
    }   
        
    draw_grid(50.0, BLACK);
    next_frame().await;
}
}