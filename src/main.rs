use crate::components::Wire;
use core::marker::PhantomData;
use macroquad::prelude::*;
use specs::prelude::*;

pub mod components;
mod systems;
use components::nodes::{self, add_node_systems};
use components::Connected;
use systems::WireSys;

// #[macroquad::main("SIMple Electronics")]
// async fn main() {
fn main() {
    let mut world = World::new();

    let mut dispatcher = {
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder = add_node_systems(dispatcher_builder);

        dispatcher_builder.build()
    };
    dispatcher.setup(&mut world);

    let wire_1 = world.create_entity().with(Wire::default()).build();
    let wire_2 = world.create_entity().with(Wire::default()).build();
    let wire_3 = world.create_entity().with(Wire::default()).build();

    world
        .create_entity()
        .with(Connected {
            node: PhantomData::<nodes::OnNode>,
            inputs: [],
            outputs: [Some(wire_1)],
        })
        .build();

    world
        .create_entity()
        .with(Connected {
            node: PhantomData::<nodes::OnNode>,
            inputs: [],
            outputs: [Some(wire_2)],
        })
        .build();

    world
        .create_entity()
        .with(Connected {
            node: PhantomData::<nodes::AndNode>,
            inputs: [Some(wire_1), Some(wire_2)],
            outputs: [Some(wire_3)],
        })
        .build();

    for _ in 0..3 {
        dispatcher.dispatch(&world);
        println!("--------------------------------")
        // next_frame().await;
    }
}
