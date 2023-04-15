use macroquad::prelude::*;

const PLAYER_JUMP_STRENGTH: f32 = 400f32;
const MAX_GRAVITY: f32 = 2500f32;
const GRAVITY_ACCELERATION: f32 = 700f32;

#[derive(Debug, Clone)]
pub struct Player {
    pub texture: Texture2D,
    pub pos: Vec2,
    pub size: Vec2,
    pub gravity: f32,
    pub pipes: Vec<Pipe>,
    pub rect: Rect,
    pub is_alive: bool,
    pub rotation: f32,
}

impl Player {
    pub fn new(texture: Texture2D, pos: Vec2, pipes: Vec<Pipe>) -> Self {
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

    pub fn update(&mut self, delta: f32) {
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

    pub fn draw(&self) {
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
pub struct Pipe {
    pub texture: Texture2D,
    pub pos: Vec2,
    pub size: Vec2,
    pub rect: Rect,
    pub speed: f32,
    pub passed_player: bool,
}

impl Pipe {
    pub fn new(texture: Texture2D, pos: Vec2, speed: f32) -> Self {
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
    pub fn update(&mut self, delta: f32) {
        // movement code
        self.pos.x -= self.speed * delta;

        // after movement update rect
        self.rect.x = self.pos.x;
        self.rect.y = self.pos.y;
    }
    pub fn draw(&self) {
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }
}
