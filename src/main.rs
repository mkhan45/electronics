use crate::components::Wire;
use crate::resources::UiSignal;
use core::marker::PhantomData;
use egui_macroquad;
use macroquad::prelude::*;
use specs::prelude::*;

mod components;
mod resources;
mod svg;
mod systems;
mod ui;

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

    let and_svg = svg::texture_from_file("resources/and_gate.svg", 75, 50, mq_ctx).await;
    textures.0.insert("AND_GATE".to_owned(), and_svg);

    let xor_svg = svg::texture_from_file("resources/xor_gate.svg", 200, 175, mq_ctx).await;
    textures.0.insert("XOR_GATE".to_owned(), xor_svg);

    world.insert(textures);

    dispatcher.setup(&mut world);
    draw_dispatcher.setup(&mut world);
    world.insert(resources::AddingNode(None));
    world.insert(resources::UiSignals(Vec::new()));

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
            node: PhantomData::<nodes::OffNode>,
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

    world.insert(resources::Tick(0));
    let tick_frames = 144;
    loop {
        clear_background(BLACK);
        let i = world.fetch::<resources::Tick>().0;

        world.insert(resources::TickProgress(
            (i % tick_frames) as f64 / tick_frames as f64,
        ));
        if i % tick_frames == 0 {
            dispatcher.dispatch_seq(&world);
        }
        draw_dispatcher.dispatch_thread_local(&world);

        world.maintain();

        {
            let signals_res = world.fetch::<resources::UiSignals>();
            let ui_signals = signals_res.0.clone();
            std::mem::drop(signals_res);

            ui_signals.iter().for_each(|signal| match signal {
                UiSignal::AddNode(ty) => world.insert(resources::AddingNode(Some(*ty))),
            });
            world.insert(resources::UiSignals(Vec::new()));
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::TopPanel::top("SIMple Electronics").show(egui_ctx, |ui| {
                ui::top_panel::render_top_panel(ui, &mut world);
            });
        });
        egui_macroquad::draw();

        next_frame().await;
        world.fetch_mut::<resources::Tick>().incr();
    }
}
