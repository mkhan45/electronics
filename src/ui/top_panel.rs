use crate::resources;
use crate::systems::place_node_sys::PlaceNodeSys;
use crate::ResetSys;
use crate::{components::nodes, UiSignal};
use egui::menu;
use macroquad::prelude::*;
use specs::prelude::*;

pub fn render_top_panel(ui: &mut egui::Ui, world: &mut World) {
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

            node_button!("On Node", OnNode);
            node_button!("Off Node", OffNode);
            node_button!("Not Node", NotNode);
            node_button!("And Node", AndNode);
            node_button!("Or Node", OrNode);
            node_button!("Nand Node", NandNode);
            node_button!("Nor Node", NorNode);
            node_button!("Xor Node", XorNode);
            node_button!("Xnor Node", XnorNode);
        });

        if ui.button("Reset").clicked() || is_key_pressed(KeyCode::Space) {
            ResetSys.run_now(&world);
            world.insert(resources::Tick(0));
        }
    });
}
