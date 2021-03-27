use macroquad::prelude::*;

#[macroquad::main("Bouncy Ball")]
async fn main() {
    let (mut x, mut y) = (screen_width() / 2.0, screen_height() / 2.0);
    let (mut x_vel, mut y_vel) = (5.0, -2.0);
    let radius = screen_height() / 30.0;

    loop {
        clear_background(BLACK);
        draw_circle(x, y, radius, WHITE);

        let (left, right) = (x - radius, x + radius);

        let (top, bottom) = (y - radius, y + radius);

        if (left <= 0.0 && x_vel <= 0.0) || (right >= screen_width() && x_vel >= 0.0) {
            x_vel *= -1.0;
        }
        if (top <= 0.0 && y_vel <= 0.0) || (bottom >= screen_height() && y_vel >= 0.0) {
            y_vel *= -1.0;
        }

        x += x_vel;
        y += y_vel;

        next_frame().await
    }
}
