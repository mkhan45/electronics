use crate::components::Wire;
use core::marker::PhantomData;
use draw_sys::add_draw_system;
use macroquad::prelude::*;
use specs::prelude::*;

pub mod components;
mod systems;
use components::{
    nodes::{self, add_node_systems},
    Orientation,
};
use components::{Connected, Pos};
use systems::WireSys;

mod draw_sys;

#[macroquad::main("SIMple Electronics")]
async fn main() {
    // fn main() {
    let mut world = World::new();

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

    dispatcher.setup(&mut world);
    draw_dispatcher.setup(&mut world);

    let wire_1 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(150.0, screen_height() / 2.0 - 50.0),
        })
        .build();
    let wire_2 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(150.0, screen_height() / 2.0 + 50.0),
        })
        .build();
    let wire_3 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(650.0, screen_height() / 2.0),
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
            node: PhantomData::<nodes::OnNode>,
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
            node: PhantomData::<nodes::AndNode>,
            inputs: [Some(wire_1), Some(wire_2)],
            outputs: [Some(wire_3)],
        })
        .with(Pos {
            orientation: Orientation::Right,
            pos: Vec2::new(350.0, screen_height() / 2.0),
        })
        .build();

    // for _ in 0..3 {
    let mut i = 0;
    loop {
        if i % 72 == 1 {
            dispatcher.dispatch(&world);
            println!("--------------------------------");
        }
        draw_dispatcher.dispatch(&world);

        if is_key_pressed(KeyCode::Space) {
            crate::systems::ResetSys.run_now(&world);
        }

        world.maintain();

        next_frame().await;
        i += 1;
    }
}
