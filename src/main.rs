use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Player {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (is_key_down(KeyCode::A), is_key_down(KeyCode::D)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };

        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

struct Block {
    rect: Rect,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}

#[macroquad::main("breakout")]
async fn main() {
    let mut player = Player::new();

    let mut blocks = Vec::new();
    let (width, height) = (5, 5);
    let padding = 6f32;
    let total_block_size = BLOCK_SIZE + Vec2::from_array([padding, padding]);
    let blocks_start_pos = Vec2::from_array([
        (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
        50f32,
    ]);

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(
            blocks_start_pos + Vec2::from_array([block_x, block_y]),
        ));
    }

    loop {
        player.update(get_frame_time());

        clear_background(WHITE);

        player.draw();

        for block in blocks.iter() {
            block.draw();
        }

        next_frame().await
    }
}
