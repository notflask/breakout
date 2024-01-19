use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;

pub fn draw_title_text(text: &str, font: Option<&Font>) {
    let dims = measure_text(text, font, 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams {
            font,
            font_size: 50u16,
            color: BLACK,
            ..Default::default()
        },
    )
}

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

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

#[derive(PartialEq, Eq)]
pub enum BlockType {
    Regular,
    SpawnBall,
}

struct Block {
    rect: Rect,
    lives: i32,
    block_type: BlockType,
}

impl Block {
    pub fn new(pos: Vec2, block_type: BlockType) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 3,
            block_type,
        }
    }

    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => match self.lives {
                2 => ORANGE,
                1 => RED,
                _ => GREEN,
            },
            BlockType::SpawnBall => PINK,
        };

        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

pub struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: Vec2::from_array([rand::gen_range(-1f32, 1f32), 1f32]),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }

        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };

    let to = b.center() - a.center();
    let signum = to.signum();

    match intersection.w > intersection.h {
        true => {
            a.y -= signum.y * intersection.h;
            vel.y = -signum.y * vel.y.abs();
        }
        false => {
            a.x -= signum.x * intersection.w;
            vel.x = -signum.x * vel.x.abs();
        }
    }

    true
}

fn reset_game(
    score: &mut i32,
    player_lives: &mut i32,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>,
    player: &mut Player,
) {
    *player = Player::new();
    *score = 0;
    *player_lives = 3;
    balls.clear();
    balls.push(Ball::new(Vec2::from_array([
        screen_width() * 0.5f32 - BALL_SIZE * 0.5f32,
        screen_height() * 0.5f32,
    ])));

    blocks.clear();
    init_blocks(blocks);
}

fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (5, 5);
    let padding = 6f32;
    let total_block_size = BLOCK_SIZE + Vec2::from_array([padding, padding]);
    let blocks_start_pos = Vec2::from_array([
        (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
        60f32,
    ]);
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(
            blocks_start_pos + Vec2::from_array([block_x, block_y]),
            BlockType::Regular,
        ));
    }

    for _ in 0..4 {
        let index = rand::gen_range(0, blocks.len());
        blocks[index].block_type = BlockType::SpawnBall;
    }
}

#[macroquad::main("breakout")]
async fn main() {
    let font = load_ttf_font("res/Heebo-VariableFont_wght.ttf")
        .await
        .unwrap();

    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_lives = 3;

    let mut player = Player::new();
    let mut balls = Vec::new();

    let mut blocks = Vec::new();
    init_blocks(&mut blocks);

    balls.push(Ball::new(Vec2::from_array([
        screen_width() * 0.5f32 - BALL_SIZE * 0.5f32,
        screen_height() * 0.5f32,
    ])));

    loop {
        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {
                player.update(get_frame_time());

                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }

                let mut to_spawn = vec![];
                for ball in balls.iter_mut() {
                    resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);

                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                            block.lives -= 1;

                            if block.lives <= 0 {
                                score += 10;
                                if block.block_type == BlockType::SpawnBall {
                                    to_spawn.push(Ball::new(ball.rect.point()));
                                }
                            }
                        }
                    }
                }

                for ball in to_spawn.into_iter() {
                    balls.push(ball);
                }

                let balls_len = balls.len();
                balls.retain(|ball| ball.rect.y < screen_height());
                let removed_balls = balls_len - balls.len();

                if removed_balls > 0 && balls.is_empty() {
                    player_lives -= 1;

                    balls.push(Ball::new(
                        player.rect.point()
                            + Vec2::from_array([
                                player.rect.w * 0.5f32 + BALL_SIZE * 0.5f32,
                                -50f32,
                            ]),
                    ));

                    if player_lives <= 0 {
                        game_state = GameState::Dead;
                    }
                }

                blocks.retain(|block| block.lives > 0);

                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
            }
            GameState::LevelCompleted => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(
                        &mut score,
                        &mut player_lives,
                        &mut blocks,
                        &mut balls,
                        &mut player,
                    );
                }
            }
            GameState::Dead => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(
                        &mut score,
                        &mut player_lives,
                        &mut blocks,
                        &mut balls,
                        &mut player,
                    );
                }
            }
        }

        clear_background(WHITE);

        player.draw();

        for block in blocks.iter() {
            block.draw();
        }

        for ball in balls.iter() {
            ball.draw();
        }

        match game_state {
            GameState::Menu => {
                draw_title_text("Press SPACE to start", Some(&font));
            }
            GameState::Game => {
                let score_text = &format!("Score:  {}", score);
                let score_text_size = measure_text(&score_text, Some(&font), 30u16, 1.0);

                draw_text_ex(
                    score_text,
                    screen_width() * 0.5f32 - score_text_size.width * 0.5f32,
                    40.0,
                    TextParams {
                        font: Some(&font),
                        font_size: 30u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );

                draw_text_ex(
                    &format!("Lives left:  {}", player_lives),
                    30.0,
                    40.0,
                    TextParams {
                        font: Some(&font),
                        font_size: 30u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
            GameState::LevelCompleted => {
                draw_title_text(&format!("You win! Final score: {}", score), Some(&font));
            }
            GameState::Dead => {
                draw_title_text(&format!("You lost! Score: {}", score), Some(&font));
            }
        }

        next_frame().await
    }
}
