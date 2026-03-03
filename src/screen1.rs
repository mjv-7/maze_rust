use macroquad::prelude::*;
use crate::modules::text_button::TextButton;
use crate::modules::grid::{draw_grid};
use crate::modules::label::Label;


pub async fn run() -> String {
    let mut lbl_out = Label::new("Hello World", 600.0, 50.0, 30);
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
    
    draw_text("Screen 1", 20.0, 40.0, 30.0, WHITE);
    lbl_out.draw();
    
    if btn_text.click() || is_key_down(KeyCode::Space) {
        return "screen2".to_string();
    }
    
    if btn_exit.click() {
        std::process::exit(0);
    }
    
    draw_grid(50.0, BLACK);
    next_frame().await;
}
}