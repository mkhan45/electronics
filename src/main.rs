use crate::nodes::Wire;
use crate::resources::UiSignal;
use macroquad::prelude::*;
use resources::CameraRes;
use specs::prelude::*;

mod components;
mod resources;
// mod scripting;
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
    world.insert(resources::UiSignals(Vec::new()));
    world.insert(resources::GridMode::default());

    world.insert(resources::Tick(0));
    world.insert(resources::TickFrames(60));
    world.insert(resources::CameraRes::default());
    world.insert(resources::RhaiEngine::default());
    world.insert(resources::RhaiScope::default());
    world.insert(resources::CreatingCompoundNode::default());

    let mut prev_mouse_pos = {
        let (mx, my) = mouse_position();
        Vec2::new(mx, my)
    };

    let mut last_fps = [60i32; 256];

    // let script: String = macroquad::file::load_string("test_scripts/basic_circuit.rhai")
    //     .await
    //     .unwrap();

    // scripting::run_circuit_create_sys(script, &world);
    // world.maintain();

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
                UiSignal::AddNode(ty) => world.insert(resources::UIState::AddingNode(*ty)),
                UiSignal::Delete => world.insert(resources::UIState::Deleting),
                UiSignal::CreateNode => {
                    world.insert(resources::UIState::Nothing);
                    let compound_node = world
                        .create_entity()
                        .with(components::CompoundNode::default())
                        .build();
                    world.insert(resources::CreatingCompoundNode(Some(compound_node)));
                }
                UiSignal::SaveCompoundNode => todo!(),
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

        {
            let camera = world.fetch::<CameraRes>().0;
            world.insert(resources::MousePos(
                camera.screen_to_world(mouse_position().into()),
            ));
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            ui::mouse_click::handle_mouse_click(&mut world);
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            ui::mouse_click::handle_mouse_right_click(&mut world);
        }

        let new_mouse_pos = {
            let (mx, my) = mouse_position();
            Vec2::new(mx, -my)
        };

        if is_mouse_button_down(MouseButton::Middle) {
            world.fetch_mut::<CameraRes>().0.offset += (new_mouse_pos - prev_mouse_pos) / 1000.0;
        }

        {
            let mp: Vec2 = mouse_position().into();
            let old_camera = world.fetch::<CameraRes>().0;
            let old_focus = old_camera.screen_to_world(mp);

            let mwheel = macroquad::input::mouse_wheel().1;
            let zoom_fac = 1.0 + mwheel / 10.0;
            world.fetch_mut::<CameraRes>().0.zoom *= zoom_fac;
            let new_camera = world.fetch::<CameraRes>().0;
            let new_focus = new_camera.screen_to_world(mp);

            let delta_focus = new_focus - old_focus;
            world.fetch_mut::<CameraRes>().0.offset += delta_focus * new_camera.zoom;
        }

        macroquad::camera::set_camera(world.fetch::<CameraRes>().0);
        prev_mouse_pos = new_mouse_pos;

        egui_macroquad::draw();

        next_frame().await;
    }
}
