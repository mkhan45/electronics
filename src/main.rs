use crate::components::Wire;
use core::marker::PhantomData;
use macroquad::prelude::*;
use specs::prelude::*;

pub mod components;
mod systems;
use components::nodes;
use components::Connected;
use systems::{ElectroSys, WireSys};

// #[macroquad::main("SIMple Electronics")]
// async fn main() {
fn main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(WireSys, "wire_sys", &[])
        .with(
            ElectroSys::<nodes::OnNode, 0, 1>::default(),
            "on_node_sys",
            &["wire_sys"],
        )
        .with(
            ElectroSys::<nodes::OffNode, 0, 1>::default(),
            "off_node_sys",
            &["wire_sys"],
        )
        .with(
            ElectroSys::<nodes::NotNode, 1, 1>::default(),
            "not_node_sys",
            &["wire_sys"],
        )
        .build();
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
            node: PhantomData::<nodes::NotNode>,
            inputs: [Some(wire_1)],
            outputs: [Some(wire_2)],
        })
        .build();

    world
        .create_entity()
        .with(Connected {
            node: PhantomData::<nodes::NotNode>,
            inputs: [Some(wire_2)],
            outputs: [Some(wire_3)],
        })
        .build();

    for _ in 0..3 {
        dispatcher.dispatch(&mut world);
        println!("--------------------------------")
        // next_frame().await;
    }
}
