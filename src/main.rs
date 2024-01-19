use macroquad::prelude::*;

#[macroquad::main("breakout")]
async fn main() {
    loop {
        clear_background(WHITE);
        next_frame().await
    }
}
