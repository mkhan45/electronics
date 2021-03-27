use macroquad::prelude::*;
use specs::{prelude::*, Component};

#[derive(Component)]
struct Bird {
    pub y_pos: f32,
    pub y_vel: f32,
}

impl Default for Bird {
    fn default() -> Self {
        Bird {
            y_pos: screen_height() / 3.0,
            y_vel: -50.0,
        }
    }
}

// just store the coordinates of the gap
#[derive(Component)]
struct PipePair {
    x: f32,
    y: f32,
}

impl PipePair {
    fn new(x: f32) -> Self {
        let y = rand::gen_range(screen_height() / 4.0, screen_height() * 3.0 / 4.0);
        PipePair { x, y }
    }

    fn reset(&mut self) {
        self.y = rand::gen_range(screen_height() / 4.0, screen_height() * 3.0 / 4.0);
        self.x = screen_width();
    }
}

fn pipe_gap() -> f32 {
    screen_height() / 4.25
}

fn pipe_width() -> f32 {
    screen_width() / 20.0
}

fn pipe_speed() -> f32 {
    screen_width() / 2.0
}

fn bird_x_pos() -> f32 {
    screen_width() / 15.0
}

fn bird_size() -> f32 {
    screen_height() / 30.0
}

struct DrawSys;
impl<'a> System<'a> for DrawSys {
    type SystemData = (Read<'a, Bird>, ReadStorage<'a, PipePair>);

    fn run(&mut self, (bird, pipes): Self::SystemData) {
        clear_background(BLACK);
        draw_circle(bird_x_pos(), bird.y_pos, bird_size(), RED);

        pipes.join().for_each(|PipePair { x, y }| {
            // top pipe
            let h = screen_height();
            let gap = pipe_gap();
            draw_rectangle(*x, y - h - gap / 2.0, pipe_width(), h, GREEN);
            draw_rectangle(*x, *y + gap / 2.0, pipe_width(), h, GREEN);
        });
    }
}

struct BirdPhysicsSys;
impl<'a> System<'a> for BirdPhysicsSys {
    type SystemData = Write<'a, Bird>;

    fn run(&mut self, mut bird: Self::SystemData) {
        let dt = macroquad::time::get_frame_time();
        bird.y_vel += 1800.0 * dt;
        bird.y_pos += bird.y_vel * dt;
    }
}

struct BirdInputSys;
impl<'a> System<'a> for BirdInputSys {
    type SystemData = Write<'a, Bird>;

    fn run(&mut self, mut bird: Self::SystemData) {
        if is_key_pressed(KeyCode::Space) {
            bird.y_vel -= 1000.0;
        }
    }
}

struct PipeMoveSys;
impl<'a> System<'a> for PipeMoveSys {
    type SystemData = WriteStorage<'a, PipePair>;

    fn run(&mut self, mut pipes: Self::SystemData) {
        (&mut pipes).join().for_each(|pair| {
            let dt = macroquad::time::get_frame_time();
            pair.x -= pipe_speed() * dt;
            if pair.x + pipe_width() < 0.0 {
                pair.reset();
            }
        });
    }
}

struct BirdCollideSys;
impl<'a> System<'a> for BirdCollideSys {
    type SystemData = (Read<'a, Bird>, ReadStorage<'a, PipePair>);

    fn run(&mut self, (bird, pipes): Self::SystemData) {
        pipes.join().for_each(|pair| {
            if bird_x_pos() + bird_size() / 2.0 > pair.x
                && (bird.y_pos - pair.y).abs() > (pipe_gap() - bird_size()) / 2.0
            {
                use std::process;
                process::exit(0);
            }
        });
    }
}

#[macroquad::main("Bouncy Ball")]
async fn main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(DrawSys, "draw_sys", &[])
        .with(BirdPhysicsSys, "bird_physics_sys", &[])
        .with(BirdInputSys, "bird_input_sys", &["bird_physics_sys"])
        .with(BirdCollideSys, "bird_collide_sys", &[])
        .with(PipeMoveSys, "pipe_move_sys", &[])
        .build();

    dispatcher.setup(&mut world);

    world.insert(Bird::default());

    world
        .create_entity()
        .with(PipePair::new(screen_width()))
        .build();
    world
        .create_entity()
        .with(PipePair::new(screen_width() * 1.5))
        .build();

    loop {
        dispatcher.dispatch(&mut world);
        next_frame().await;
    }
}
