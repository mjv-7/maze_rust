use crate::modules::collision::check_collision;
use crate::modules::grid::draw_grid;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::text_button::TextButton;
use crate::modules::still_image::StillImage;
use crate::modules::label::Label;
use macroquad::prelude::*;


pub async fn run() -> String {
    use_virtual_resolution(1440.0, 1080.0);
    let mut lbl_out = Label::new("Thank yuou for playingn\nBeautiful Maze Game", 600.0, 50.0, 30);
    lbl_out.with_colors(WHITE, Some(DARKGRAY));
    let img_bg = StillImage::new
    ("assets/engpage.png", 
    1440.0, 
    1080.0, 
    0.0, 
    0.0, 
    true, 
    1.0).await;
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
    img_bg.draw();
    lbl_out.draw();
    if btn_exit.click() {
        std::process::exit(0);
    }
    draw_text("By Mujibullah", 20.0, 40.0, 30.0, WHITE);
    draw_grid(50.0, BLACK);
    next_frame().await;
}
}