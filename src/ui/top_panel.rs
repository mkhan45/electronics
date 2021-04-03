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

        if is_mouse_button_pressed(MouseButton::Left) {
            let node = world.fetch::<resources::AddingNode>();

            if let resources::AddingNode(Some(n)) = &*node {
                macro_rules! place_node_systems {
                    ( $([$node:ident, $i:expr, $o:expr]),* $(,)? ) => {
                        #[allow(unreachable_patterns)]
                        match n {
                            $(nodes::NodeTy::$node => {
                                PlaceNodeSys::<nodes::$node, $i, $o>::default().run_now(&world)
                            })*
                            _ => todo!(),
                        }
                    };
                }

                place_node_systems!(
                    [OnNode, 0, 1],
                    [OffNode, 0, 1],
                    [NotNode, 1, 1],
                    [AndNode, 2, 1],
                    [OrNode, 2, 1],
                    [NandNode, 2, 1],
                    [NorNode, 2, 1],
                    [XorNode, 2, 1],
                    [XnorNode, 2, 1]
                );

                std::mem::drop(node);
                world.insert(resources::AddingNode(None));
            }
        }
    });
}
