use macroquad::prelude::*;
use std::time::Instant;

const PADDLE_HEIGHT: f32 = 200.0;
const PADDLE_WIDTH: f32 = 20.0;
const BALL_RADIUS: f32 = 15.0;

const PADDLE_WALL_OFFSET: f32 = 50.0;

const PADDLE_MOVE_SPEED: f32 = 500.0;
const BALL_MOVE_SPEED: f32 = 500.0;

const LINE_RATE: f32 = 50.0;

fn clamp_paddle(position: f32) -> f32 {
    if position < 0.0 {
        0.0
    } else if position + PADDLE_HEIGHT > screen_height() {
        screen_height() - PADDLE_HEIGHT
    } else {
        position
    }
}

fn draw_center_line() {
    let mut position: f32 = LINE_RATE / 2.0;

    let x_coord = screen_width() / 2.0;

    while position < screen_height() {
        draw_line(x_coord, position, x_coord, position + LINE_RATE, 5.0, WHITE);

        position += LINE_RATE * 2.0;
    }
}

#[macroquad::main("Pong")]
async fn main() {
    let mut left_score: u32 = 0;
    let mut right_score: u32 = 0;

    let mut left_paddle_position = screen_height() / 2.0 - PADDLE_HEIGHT / 2.0;
    let mut right_paddle_position = screen_height() / 2.0 - PADDLE_HEIGHT / 2.0;

    let mut ball_x_velocity = 0.0;
    let mut ball_y_velocity = 0.0;
    let mut ball_x = screen_width() / 2.0;
    let mut ball_y = screen_height() / 2.0;

    let mut now = Instant::now();

    loop {
        let delta_time = now.elapsed().as_millis() as f32 / 1000.0;

        clear_background(BLACK);

        if is_key_down(KeyCode::W) {
            left_paddle_position -= PADDLE_MOVE_SPEED * delta_time;
        } else if is_key_down(KeyCode::S) {
            left_paddle_position += PADDLE_MOVE_SPEED * delta_time;
        }

        if is_key_down(KeyCode::Up) {
            right_paddle_position -= PADDLE_MOVE_SPEED * delta_time;
        } else if is_key_down(KeyCode::Down) {
            right_paddle_position += PADDLE_MOVE_SPEED * delta_time;
        }

        if is_key_pressed(KeyCode::Space) {
            ball_x = screen_width() / 2.0;
            ball_y = screen_height() / 2.0;

            if rand::gen_range(-1.0, 1.0) >= 0.0 {
                ball_x_velocity = BALL_MOVE_SPEED;
            } else {
                ball_x_velocity = -BALL_MOVE_SPEED;
            }

            if rand::gen_range(-1.0, 1.0) >= 0.0 {
                ball_y_velocity = BALL_MOVE_SPEED;
            } else {
                ball_y_velocity = -BALL_MOVE_SPEED;
            }
        }

        if ball_x < 0.0 {
            right_score += 1;

            ball_x = screen_width() / 2.0;
            ball_y = screen_height() / 2.0;

            ball_x_velocity = 0.0;
            ball_y_velocity = 0.0;
        } else if ball_x > screen_width() {
            left_score += 1;

            ball_x = screen_width() / 2.0;
            ball_y = screen_height() / 2.0;

            ball_x_velocity = 0.0;
            ball_y_velocity = 0.0;
        }

        left_paddle_position = clamp_paddle(left_paddle_position);
        right_paddle_position = clamp_paddle(right_paddle_position);

        ball_x += ball_x_velocity * delta_time;
        ball_y += ball_y_velocity * delta_time;

        if ball_y - BALL_RADIUS < 0.0 {
            ball_y_velocity *= -1.0;
            ball_y = BALL_RADIUS;
        } else if ball_y + BALL_RADIUS > screen_height() {
            ball_y_velocity *= -1.0;
            ball_y = screen_height() - BALL_RADIUS;
        }

        if ball_x - BALL_RADIUS < PADDLE_WALL_OFFSET + PADDLE_WIDTH &&
            ball_x + BALL_RADIUS > PADDLE_WALL_OFFSET &&
            ball_y > left_paddle_position &&
            ball_y < left_paddle_position + PADDLE_HEIGHT {
                ball_x_velocity *= -1.0;
                ball_x = PADDLE_WALL_OFFSET + PADDLE_WIDTH + BALL_RADIUS;
            }
        else if ball_x + BALL_RADIUS > screen_width() - PADDLE_WALL_OFFSET - PADDLE_WIDTH &&
            ball_x - BALL_RADIUS < screen_width() - PADDLE_WALL_OFFSET &&
            ball_y > right_paddle_position &&
            ball_y < right_paddle_position + PADDLE_HEIGHT {
                ball_x_velocity *= -1.0;
                ball_x = screen_width() - (PADDLE_WALL_OFFSET + PADDLE_WIDTH + BALL_RADIUS);
            }

        draw_center_line();

        draw_rectangle(
            PADDLE_WALL_OFFSET,
            left_paddle_position,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            WHITE
        );

        draw_rectangle(
            screen_width() - PADDLE_WALL_OFFSET - PADDLE_WIDTH,
            right_paddle_position,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            WHITE
        );

        draw_circle(ball_x, ball_y, BALL_RADIUS, WHITE);

        let left_score_width = measure_text(left_score.to_string().as_str(), None, 100, 1.0).width;
        draw_text(left_score.to_string().as_str(), screen_width() / 2.0 - 50.0 - left_score_width, 100.0, 100.0, WHITE);
        draw_text(right_score.to_string().as_str(), screen_width() / 2.0 + 50.0, 100.0, 100.0, WHITE);

        now = Instant::now();
        next_frame().await
    }
}