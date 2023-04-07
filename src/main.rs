use macroquad::prelude::*;
use std::time::Instant;

mod icons;

const PADDLE_HEIGHT: f32 = 200.;
const PADDLE_WIDTH: f32 = 20.;
const BALL_RADIUS: f32 = 17.5;

const PADDLE_WALL_OFFSET: f32 = 50.;

const PADDLE_MOVE_SPEED: f32 = 600.;
const BALL_MOVE_SPEED: f32 = 500.;
const BALL_ACCELERATION: f32 = 50.;

const LINE_RATE: f32 = 50.;

fn window_config() -> Conf {
    let icon = miniquad::conf::Icon {
        small: icons::ICON_SMALL,
        medium: icons::ICON_MEDIUM,
        big: icons::ICON_BIG
    };

    Conf {
        window_title: "Pong!".to_owned(),
        window_width: 1200,
        window_height: 800,
        window_resizable: false,
        icon: Some(icon),
        ..Default::default()
    }
}

fn clamp_paddle(position: f32) -> f32 {
    if position < 0. {
        0.
    } else if position + PADDLE_HEIGHT > screen_height() {
        screen_height() - PADDLE_HEIGHT
    } else {
        position
    }
}

fn draw_center_line() {
    let mut position: f32 = LINE_RATE / 2.;

    let x_coord = screen_width() / 2.;

    while position < screen_height() {
        draw_line(x_coord, position, x_coord, position + LINE_RATE, 5., WHITE);

        position += LINE_RATE * 2.;
    }
}

fn draw_ball(x: f32, y: f32) {
    let diameter = BALL_RADIUS * 2.;

    draw_rectangle(x - diameter/2., y - diameter/7.*1.5, diameter, diameter/7.*3., WHITE);
    draw_rectangle(x - diameter/7.*1.5, y - diameter/2., diameter/7.*3., diameter, WHITE);
    draw_rectangle(x - diameter/7.*2.5, y - diameter/7.*2.5, diameter/7.*5., diameter/7.*5., WHITE);
}

#[macroquad::main(window_config)]
async fn main() {
    let mut left_score: u32 = 0;
    let mut right_score: u32 = 0;

    let mut left_paddle_position = screen_height() / 2. - PADDLE_HEIGHT / 2.;
    let mut right_paddle_position = screen_height() / 2. - PADDLE_HEIGHT / 2.;
    let mut left_paddle_velocity;
    let mut right_paddle_velocity;
    let mut last_left_paddle_velocity = 0.;
    let mut last_right_paddle_velocity = 0.;

    let mut ball_x = screen_width() / 2.;
    let mut ball_y = screen_height() / 2.;
    let mut ball_x_velocity = 0.;
    let mut ball_y_velocity = 0.;

    let mut show_fps = false;
    let mut last_fps = get_fps();

    let mut now = Instant::now();

    loop {
        let delta_time = now.elapsed().as_millis() as f32 / 1000.;

        clear_background(BLACK);

        if is_key_down(KeyCode::W) {
            left_paddle_velocity = -PADDLE_MOVE_SPEED * delta_time;
        } else if is_key_down(KeyCode::S) {
            left_paddle_velocity = PADDLE_MOVE_SPEED * delta_time;
        } else {
            left_paddle_velocity = 0.;
        }

        if is_key_down(KeyCode::Up) {
            right_paddle_velocity = -PADDLE_MOVE_SPEED * delta_time;
        } else if is_key_down(KeyCode::Down) {
            right_paddle_velocity = PADDLE_MOVE_SPEED * delta_time;
        } else {
            right_paddle_velocity = 0.;
        }

        if is_key_pressed(KeyCode::Space) {
            ball_x = screen_width() / 2.;
            ball_y = screen_height() / 2.;

            if rand::gen_range(-1., 1.) >= 0. {
                ball_x_velocity = BALL_MOVE_SPEED;
            } else {
                ball_x_velocity = -BALL_MOVE_SPEED;
            }

            if rand::gen_range(-1., 1.) >= 0. {
                ball_y_velocity = BALL_MOVE_SPEED;
            } else {
                ball_y_velocity = -BALL_MOVE_SPEED;
            }
        }

        if is_key_pressed(KeyCode::F) {
            show_fps = !show_fps;
        }

        if ball_x < 0. {
            right_score += 1;

            ball_x = screen_width() / 2.;
            ball_y = screen_height() / 2.;

            ball_x_velocity = 0.;
            ball_y_velocity = 0.;
        } else if ball_x > screen_width() {
            left_score += 1;

            ball_x = screen_width() / 2.;
            ball_y = screen_height() / 2.;

            ball_x_velocity = 0.;
            ball_y_velocity = 0.;
        }

        left_paddle_velocity = (last_left_paddle_velocity + left_paddle_velocity) / 2.;
        right_paddle_velocity = (last_right_paddle_velocity + right_paddle_velocity) / 2.;

        left_paddle_position += left_paddle_velocity;
        right_paddle_position += right_paddle_velocity;

        last_left_paddle_velocity = left_paddle_velocity;
        last_right_paddle_velocity = right_paddle_velocity;

        left_paddle_position = clamp_paddle(left_paddle_position);
        right_paddle_position = clamp_paddle(right_paddle_position);

        ball_x += ball_x_velocity * delta_time;
        ball_y += ball_y_velocity * delta_time;

        if ball_y - BALL_RADIUS < 0. {
            ball_y_velocity *= -1.;
            ball_y = BALL_RADIUS;
        } else if ball_y + BALL_RADIUS > screen_height() {
            ball_y_velocity *= -1.;
            ball_y = screen_height() - BALL_RADIUS;
        }

        if ball_x - BALL_RADIUS < PADDLE_WALL_OFFSET + PADDLE_WIDTH &&
            ball_x + BALL_RADIUS > PADDLE_WALL_OFFSET &&
            ball_y > left_paddle_position &&
            ball_y < left_paddle_position + PADDLE_HEIGHT {
                ball_x_velocity *= -1.;
                ball_x = PADDLE_WALL_OFFSET + PADDLE_WIDTH + BALL_RADIUS;

                ball_x_velocity += BALL_ACCELERATION * ball_x_velocity.signum();
                ball_y_velocity += BALL_ACCELERATION * ball_y_velocity.signum();
            }
        else if ball_x + BALL_RADIUS > screen_width() - PADDLE_WALL_OFFSET - PADDLE_WIDTH &&
            ball_x - BALL_RADIUS < screen_width() - PADDLE_WALL_OFFSET &&
            ball_y > right_paddle_position &&
            ball_y < right_paddle_position + PADDLE_HEIGHT {
                ball_x_velocity *= -1.;
                ball_x = screen_width() - (PADDLE_WALL_OFFSET + PADDLE_WIDTH + BALL_RADIUS);

                ball_x_velocity += BALL_ACCELERATION * ball_x_velocity.signum();
                ball_y_velocity += BALL_ACCELERATION * ball_y_velocity.signum();
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

        draw_ball(ball_x, ball_y);

        let left_score_width = measure_text(left_score.to_string().as_str(), None, 80, 1.).width;
        draw_text(left_score.to_string().as_str(), screen_width() / 2. - 50. - left_score_width, 80., 80., WHITE);
        draw_text(right_score.to_string().as_str(), screen_width() / 2. + 50., 80., 80., WHITE);

        last_fps = (last_fps * 2 + get_fps()) / 3;
    
        if show_fps {
            draw_text(last_fps.to_string().as_str(), 20., 40., 40., WHITE);
        }

        now = Instant::now();
        next_frame().await
    }
}