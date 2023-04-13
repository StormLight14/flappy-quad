use macroquad::prelude::*;

const PIPE_SPEED: f32 = 500f32;
const PLAYER_JUMP_STRENGTH: f32 = 400f32;
const MAX_GRAVITY: f32 = 2500f32;
const GRAVITY_ACCELERATION: f32 = 700f32;

enum GameState {
    MainMenu,
    Playing,
    Dead,
}

#[derive(Debug, Clone)]
struct Player {
    texture: Texture2D,
    pos: Vec2,
    size: Vec2,
    gravity: f32,
    pipes: Vec<Pipe>,
    rect: Rect,
    is_alive: bool,
    rotation: f32,
}

impl Player {
    fn new(texture: Texture2D, pos: Vec2, pipes: Vec<Pipe>) -> Self {
        let size = Vec2::from_array([texture.width(), texture.height()]);

        Self {
            texture: texture,
            pos: pos,
            size: Vec2::from_array([texture.width(), texture.height()]),
            gravity: 0f32,
            pipes: pipes,
            rect: Rect::new(pos.x, pos.y, size.x, size.y),
            is_alive: true,
            rotation: 0f32,
        }
    }

    fn update(&mut self, delta: f32) {
        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            self.gravity = -PLAYER_JUMP_STRENGTH;
        }

        if self.gravity < MAX_GRAVITY {
            self.gravity += GRAVITY_ACCELERATION * delta;
        } else {
            self.gravity = MAX_GRAVITY;
        }

        self.rotation = (self.gravity / 250f32).sin() * 0.5;
        self.pos.y += self.gravity * delta;

        self.rect.x = self.pos.x;
        self.rect.y = self.pos.y;

        if self.pos.y > screen_height() - self.size.y {
            self.pos.y = screen_height() - self.size.y;
            self.gravity = 0f32;
            self.is_alive = false;
        }
        for pipe in self.pipes.iter() {
            if self.rect.overlaps(&pipe.rect) {
                self.is_alive = false;
            }
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.pos.x,
            self.pos.y,
            Color::new(1.0, 1.0, 1.0, 1.0),
            DrawTextureParams {
                dest_size: Some(self.size),
                source: Some(Rect::new(0.0, 0.0, self.size.x, self.size.y)),
                rotation: self.rotation,
                flip_x: false,
                flip_y: false,
                ..Default::default()
            },
        );
    }
}

#[derive(Clone, Debug)]
struct Pipe {
    texture: Texture2D,
    pos: Vec2,
    size: Vec2,
    rect: Rect,
    speed: f32,
    passed_player: bool,
}

impl Pipe {
    fn new(texture: Texture2D, pos: Vec2, speed: f32) -> Self {
        let size = Vec2::from_array([texture.width(), texture.height()]);

        Self {
            texture: texture,
            pos: pos,
            size: size,
            rect: Rect::new(pos.x, pos.y, size.x, size.y),
            speed: speed,
            passed_player: false,
        }
    }
    fn update(&mut self, delta: f32) {
        // movement code
        self.pos.x -= self.speed * delta;

        // after movement update rect
        self.rect.x = self.pos.x;
        self.rect.y = self.pos.y;
    }
    fn draw(&self) {
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }
}

fn draw_text(
    pos: (f32, f32),
    text: &str,
    font: Font,
    font_size: u16,
    font_scale: f32,
    color: Color,
) {
    let (x, y) = pos;
    let text_dim = measure_text(text, Some(font), font_size, font_scale);

    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: font,
            font_size: font_size,
            font_scale: font_scale,
            color: color,
            ..Default::default()
        },
    );
}

fn draw_title_text(text: &str, font: Font) {
    let text_dim = measure_text(text, Some(font), 25, 1f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - text_dim.width * 0.5f32,
        screen_height() * 0.5f32 - text_dim.height * 0.5f32,
        TextParams {
            font: font,
            font_size: 25,
            color: BLACK,
            ..Default::default()
        },
    )
}

fn add_pipes(mut pipes: Vec<Pipe>, texture: Texture2D, speed: f32) -> Vec<Pipe> {
    let gap_size = rand::gen_range(120f32, 160f32);
    let gap_pos = rand::gen_range(-220f32, 0f32);
    let pipe1 = Pipe::new(texture, Vec2::from_array([700f32, gap_pos]), speed);
    let pipe2 = Pipe::new(
        texture,
        Vec2::from_array([pipe1.pos.x, pipe1.pos.y + pipe1.size.y + gap_size]),
        speed,
    );
    pipes.push(pipe1);
    pipes.push(pipe2);
    pipes
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_state: GameState = GameState::MainMenu;
    let mut pipes: Vec<Pipe> = Vec::new();

    let main_font = load_ttf_font("res/fonts/minecraftia.ttf").await.unwrap();
    let player_texture = load_texture("res/player/player.png").await.unwrap();
    let pipe_texture = load_texture("res/pipe/pipe-1.png").await.unwrap();

    let mut score: f32 = 0.0;
    let mut pipe_speed = PIPE_SPEED;

    let player_pos = Vec2::from_array([100f32, 100f32]);
    let mut player = Player::new(player_texture, player_pos, pipes.clone());

    let mut counter = 50;
    loop {
        clear_background(WHITE);
        match current_state {
            GameState::Playing => {
                player.update(get_frame_time());
                if player.is_alive == false {
                    current_state = GameState::Dead;
                    player.pos = player_pos;
                    player.gravity = 0f32;
                    pipe_speed = PIPE_SPEED;
                    pipes.clear();
                }

                for pipe in pipes.iter_mut() {
                    pipe.update(get_frame_time());
                    if pipe.pos.x < player.pos.x && pipe.passed_player == false {
                        pipe.passed_player = true;
                        score += 0.5;
                        pipe_speed += 2f32;
                    }
                }
                pipes.retain(|pipe| pipe.pos.x > 0f32 - pipe.size.x);
                player.pipes = pipes.clone();

                player.draw();
                for pipe in pipes.iter() {
                    pipe.draw();
                }

                draw_text(
                    (20f32, 50f32),
                    &format!("Score: {}", score),
                    main_font,
                    20,
                    1f32,
                    Color::new(0.0, 0.0, 0.0, 1.0),
                );

                if counter == 200 {
                    pipes = add_pipes(pipes, pipe_texture, pipe_speed);
                    counter = 0;
                }
                counter += 1;
            }
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    current_state = GameState::Playing;
                }
                draw_title_text("Click or press Space to start.", main_font)
            }
            GameState::Dead => {
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    current_state = GameState::Playing;
                    score = 0f32;
                    player.is_alive = true;
                }
                draw_title_text(&format!("You died. Final score: {}", score), main_font);
            }
        };
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Quad".to_owned(),
        window_width: 640,
        window_height: 480,
        window_resizable: true,
        fullscreen: false,
        ..Default::default()
    }
}
