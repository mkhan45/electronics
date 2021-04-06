use crate::nodes::Wire;
use crate::resources::UiSignal;
use egui_macroquad;
use macroquad::prelude::*;
use specs::prelude::*;

mod components;
mod resources;
mod svg;
mod systems;
mod ui;

use components::nodes::{self, add_node_systems};
use components::{Connected, Pos};
use systems::draw_systems::add_draw_system;
use systems::simulation_systems::*;

#[macroquad::main("SIMple Electronics")]
async fn main() {
    let mut world = World::new();

    world.insert(resources::TickProgress(0.0));

    let mut dispatcher = {
        let mut builder = DispatcherBuilder::new();
        builder = add_node_systems(builder);
        builder.build()
    };

    let mut draw_dispatcher = {
        let mut builder =
            DispatcherBuilder::new().with_thread_local(systems::ui_systems::CurrentModeSys);
        builder = add_draw_system(builder);
        builder.build()
    };

    let mq_ctx = unsafe { get_internal_gl() }.quad_context;

    let mut textures = resources::Textures::default();

    let not_svg = svg::texture_from_file("resources/not_gate.svg", 50, 45, mq_ctx).await;
    textures.0.insert("NOT_GATE".to_owned(), not_svg);

    let and_svg = svg::texture_from_file("resources/and_gate.svg", 75, 50, mq_ctx).await;
    textures.0.insert("AND_GATE".to_owned(), and_svg);

    let or_svg = svg::texture_from_file("resources/or_gate.svg", 54, 43, mq_ctx).await;
    textures.0.insert("OR_GATE".to_owned(), or_svg);

    let nand_svg = svg::texture_from_file("resources/nand_gate.svg", 51, 43, mq_ctx).await;
    textures.0.insert("NAND_GATE".to_owned(), nand_svg);

    let nor_svg = svg::texture_from_file("resources/nor_gate.svg", 55, 40, mq_ctx).await;
    textures.0.insert("NOR_GATE".to_owned(), nor_svg);

    let xor_svg = svg::texture_from_file("resources/xor_gate.svg", 200, 175, mq_ctx).await;
    textures.0.insert("XOR_GATE".to_owned(), xor_svg);

    let xnor_svg = svg::texture_from_file("resources/xnor_gate.svg", 225, 175, mq_ctx).await;
    textures.0.insert("XNOR_GATE".to_owned(), xnor_svg);

    world.insert(textures);

    dispatcher.setup(&mut world);
    draw_dispatcher.setup(&mut world);
    world.insert(resources::AddingNode(None));
    world.insert(resources::AddingWire(None));
    world.insert(resources::UiSignals(Vec::new()));
    world.insert(resources::GridMode::default());

    let wire_1 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos::from_vec(Vec2::new(
            175.0,
            screen_height() / 2.0 - 30.0,
        )))
        .build();
    let wire_2 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos::from_vec(Vec2::new(
            225.0,
            screen_height() / 2.0 + 80.0,
        )))
        .build();
    let wire_3 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos::from_vec(Vec2::new(650.0, screen_height() / 3.0)))
        .build();
    let wire_4 = world
        .create_entity()
        .with(Wire::default())
        .with(Pos::from_vec(Vec2::new(650.0, 2.0 * screen_height() / 3.0)))
        .build();

    world
        .create_entity()
        .with(Connected {
            node: nodes::SwitchNode::default(),
            inputs: [],
            outputs: [Some(wire_1)],
        })
        .with(Pos::from_vec(Vec2::new(50.0, screen_height() / 2.0 - 50.0)))
        .build();

    world
        .create_entity()
        .with(Connected {
            node: nodes::SwitchNode { state: true },
            inputs: [],
            outputs: [Some(wire_2)],
        })
        .with(Pos::from_vec(Vec2::new(50.0, screen_height() / 2.0 + 50.0)))
        .build();

    world
        .create_entity()
        .with(Connected {
            node: nodes::XorNode::default(),
            inputs: [Some(wire_1), Some(wire_2)],
            outputs: [Some(wire_3)],
        })
        .with(Pos::from_vec(Vec2::new(350.0, screen_height() / 3.0)))
        .build();

    world
        .create_entity()
        .with(Connected {
            node: nodes::AndNode::default(),
            inputs: [Some(wire_2), Some(wire_1)],
            outputs: [Some(wire_4)],
        })
        .with(Pos::from_vec(Vec2::new(350.0, 2.0 * screen_height() / 3.0)))
        .build();

    world.insert(resources::Tick(0));
    world.insert(resources::TickFrames(60));

    let mut last_fps = [60i32; 256];

    loop {
        clear_background(BLACK);
        let i = world.fetch::<resources::Tick>().0;
        last_fps[i % last_fps.len()] = get_fps();

        // let tick_frames: usize = (last_fps.iter().sum::<i32>() / last_fps.len() as i32) as usize;
        let tick_frames = world.fetch::<resources::TickFrames>().0;

        world.insert(resources::TickProgress(
            (i % tick_frames) as f64 / tick_frames as f64,
        ));

        // if i > last_fps.len() && i % tick_frames == 0 {
        if i % tick_frames == 0 {
            dispatcher.dispatch_seq(&world);
        }
        draw_dispatcher.dispatch_thread_local(&world);

        world.fetch_mut::<resources::Tick>().incr();
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
            use egui::{FontDefinitions, TextStyle};
            let mut fonts = FontDefinitions::default();
            fonts.family_and_size.get_mut(&TextStyle::Button).unwrap().1 = 24.0;
            fonts.family_and_size.get_mut(&TextStyle::Body).unwrap().1 = 28.0;
            egui_ctx.set_fonts(fonts);

            egui::TopPanel::top("SIMple Electronics").show(egui_ctx, |ui| {
                ui::top_panel::render_top_panel(ui, &mut world);
            });
        });

        if is_mouse_button_pressed(MouseButton::Left) {
            ui::mouse_click::handle_mouse_click(&mut world);
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            ui::mouse_click::handle_mouse_right_click(&mut world);
        }

        egui_macroquad::draw();

        // dbg!(world.fetch::<resources::AddingWire>().0);

        next_frame().await;
    }
}
