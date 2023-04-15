use macroquad::prelude::*;

mod object;
use object::*;

const PIPE_SPEED: f32 = 500f32;

enum GameState {
    MainMenu,
    Playing,
    Dead,
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
    //let text_dim = measure_text(text, Some(font), font_size, font_scale);

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

fn add_pipes(mut pipes: Vec<Pipe>, texture: Texture2D, speed: f32, gap_scale: f32) -> Vec<Pipe> {
    let pipe_height = texture.height();

    let gap_size = rand::gen_range(120f32 * gap_scale, 160f32 * gap_scale);
    let gap_pos = rand::gen_range(
        screen_height() * 0.2 - pipe_height,
        screen_height() * 0.7 - pipe_height,
    );

    let pipe1 = Pipe::new(texture, Vec2::from_array([screen_width(), gap_pos]), speed);
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

    let mut place_counter = 50;
    let mut max_counter = 200;
    let mut gap_scale: f32 = 1.0;

    if screen_height() <= 700f32 {
        max_counter = 200;
        gap_scale = 1f32;
        println!("200");
    } else if screen_height() >= 701f32 && screen_height() <= 900f32 {
        println!("250");
        gap_scale = 1.1;
        max_counter = 250;
    } else if screen_height() >= 901f32 && screen_height() <= 1500f32 {
        println!("300");
        max_counter = 300;
        gap_scale = 1.2;
    }

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
                        pipe_speed += 4f32;
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

                if place_counter >= max_counter {
                    pipes = add_pipes(pipes, pipe_texture, pipe_speed, gap_scale);
                    place_counter = 0;
                }
                place_counter += 1;
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
