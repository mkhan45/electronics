use crate::components::Wire;
use core::marker::PhantomData;
use macroquad::prelude::*;
use specs::prelude::*;

mod components;
mod resources;
mod svg;
mod systems;

use components::{
    nodes::{self, add_node_systems},
    Orientation,
};
use components::{Connected, Pos};
use systems::draw_systems::add_draw_system;
use systems::simulation_systems::*;

#[macroquad::main("SIMple Electronics")]
async fn main() {
    // fn main() {
    let mut world = World::new();

    world.insert(resources::TickProgress(0.0));

    let mut dispatcher = {
        let mut builder = DispatcherBuilder::new();
        builder = add_node_systems(builder);
        builder.build()
    };

    let mut draw_dispatcher = {
        let mut builder = DispatcherBuilder::new();
        builder = add_draw_system(builder);
        builder.build()
    };

    let mq_ctx = unsafe { get_internal_gl() }.quad_context;

    let mut textures = resources::Textures::default();

    let and_svg = svg::texture_from_file("resources/and_gate.svg", 75, 50, mq_ctx);
    textures.0.insert("AND_GATE".to_owned(), and_svg);

    let and_svg = svg::texture_from_file("resources/xor_gate.svg", 100, 75, mq_ctx);
    textures.0.insert("XOR_GATE".to_owned(), and_svg);

    world.insert(textures);

    dispatcher.setup(&mut world);
    draw_dispatcher.setup(&mut world);

    let wire_1 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(175.0, screen_height() / 2.0 - 50.0),
        })
        .build();
    let wire_2 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(225.0, screen_height() / 2.0 + 50.0),
        })
        .build();
    let wire_3 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(650.0, screen_height() / 3.0),
        })
        .build();
    let wire_4 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(650.0, 2.0 * screen_height() / 3.0),
        })
        .build();

    world
        .create_entity()
        .with(Connected {
            node: PhantomData::<nodes::OnNode>,
            inputs: [],
            outputs: [Some(wire_1)],
        })
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(50.0, screen_height() / 2.0 - 50.0),
        })
        .build();

    world
        .create_entity()
        .with(Connected {
            node: PhantomData::<nodes::OffNode>,
            inputs: [],
            outputs: [Some(wire_2)],
        })
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(50.0, screen_height() / 2.0 + 50.0),
        })
        .build();

    world
        .create_entity()
        .with(Connected {
            node: PhantomData::<nodes::XorNode>,
            inputs: [Some(wire_1), Some(wire_2)],
            outputs: [Some(wire_3)],
        })
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(350.0, screen_height() / 3.0),
        })
        .build();

    world
        .create_entity()
        .with(Connected {
            node: PhantomData::<nodes::AndNode>,
            inputs: [Some(wire_2), Some(wire_1)],
            outputs: [Some(wire_4)],
        })
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(350.0, 2.0 * screen_height() / 3.0),
        })
        .build();

    // for _ in 0..3 {
    let mut i = 0;
    let tick_frames = 144;
    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::Space) {
            ResetSys.run_now(&world);
            i = 0;
        }

        world.insert(resources::TickProgress(
            (i % tick_frames) as f64 / tick_frames as f64,
        ));
        if i % tick_frames == 0 {
            dispatcher.dispatch(&world);
        }
        draw_dispatcher.dispatch(&world);

        world.maintain();

        next_frame().await;
        i += 1;
    }
}
