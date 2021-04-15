use crate::resources::{self, CreatingCompoundNode, GridMode};
use crate::resources::{CurrentModeText, UiSignals};
use crate::ResetSys;
use crate::{components::nodes, UiSignal};
use egui::menu;
use egui::Layout;
use macroquad::prelude::*;
use specs::prelude::*;

pub fn render_top_panel(ui: &mut egui::Ui, world: &mut World) {
    ui.horizontal(|ui| {
        ui.horizontal(|ui| {
            menu::menu(ui, "Nodes", |ui| {
                macro_rules! node_button {
                    ( $name:expr, $node:ident ) => {
                        if ui.button($name).clicked() {
                            world
                                .fetch_mut::<resources::UiSignals>()
                                .0
                                .push(UiSignal::AddNode(nodes::NodeTy::$node));
                        }
                    };
                }

                node_button!("Connection Node", Wire);
                node_button!("On Node", OnNode);
                node_button!("Off Node", OffNode);
                node_button!("Not Node", NotNode);
                node_button!("And Node", AndNode);
                node_button!("Or Node", OrNode);
                node_button!("Nand Node", NandNode);
                node_button!("Nor Node", NorNode);
                node_button!("Xor Node", XorNode);
                node_button!("Xnor Node", XnorNode);
                node_button!("Switch Node", SwitchNode);
            });

            if ui.button("Restart Sim").clicked() || is_key_pressed(KeyCode::Space) {
                ResetSys.run_now(&world);
                world.insert(resources::Tick(0));
            }

            let mut grid_mode = *world.fetch::<GridMode>();
            menu::menu(ui, "Grid Mode", |ui| {
                ui.radio_value(&mut grid_mode, GridMode::CrossHatches, "Cross Hatches");
                ui.radio_value(&mut grid_mode, GridMode::Lines, "Lines");
                ui.radio_value(&mut grid_mode, GridMode::Dots, "Dots");
                ui.radio_value(&mut grid_mode, GridMode::Off, "Off");
            });
            world.insert(grid_mode);

            if ui.button("Delete").clicked() {
                world.fetch_mut::<UiSignals>().0.push(UiSignal::Delete);
            }

            if ui.button("Remove All").clicked() {
                world.delete_all();
            }

            match world.fetch::<CreatingCompoundNode>().0 {
                Some(_) => {
                    if ui.button("Save Compound Node").clicked() {
                        world
                            .fetch_mut::<UiSignals>()
                            .0
                            .push(UiSignal::SaveCompoundNode);
                    }
                }
                None => {
                    if ui.button("Create Node").clicked() {
                        world.fetch_mut::<UiSignals>().0.push(UiSignal::CreateNode);
                    }
                }
            }
        });

        ui.with_layout(Layout::right_to_left(), |ui| {
            {
                ui.label("\t\t");
                let current_mode = world.fetch::<CurrentModeText>();
                ui.add(egui::Label::new(&current_mode.0).wrap(true));
            }

            let mut tick_frames = world.fetch::<resources::TickFrames>().0;
            ui.add(egui::Slider::usize(&mut tick_frames, 1..=512).text("Frames per Tick"));
            world.insert(resources::TickFrames(tick_frames));
        });
    });
}
