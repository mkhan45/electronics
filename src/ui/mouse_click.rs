use crate::components::Pos;
use crate::resources::UIState;
use crate::systems::place_node_sys::PlaceNodeSys;
use crate::systems::place_wire_sys::WirePlaceSys;
use macroquad::prelude::*;
use specs::prelude::*;

use crate::nodes;
pub fn handle_mouse_click(world: &mut World) {
    crate::systems::ui_systems::SwitchClickSys.run_now(world);

    let ui_state = *world.fetch::<UIState>();

    match ui_state {
        UIState::AddingWire { wire_entity, .. } => {
            // clear UIState including removing the wire entity
            let wire_placed = {
                let position_storage = world.read_storage::<Pos>();
                position_storage.get(wire_entity).is_some()
            };

            if !wire_placed {
                world.delete_entity(wire_entity).unwrap();

                crate::systems::cleanup_sys::run_cleanup_sys(world);
            }

            world.insert(UIState::Nothing);
        }
        UIState::AddingNode(n) => {
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

            use crate::all_nodes;
            all_nodes!(place_node_systems);

            world.insert(UIState::Nothing);
        }
        UIState::Nothing => {}
    }
}

pub fn handle_mouse_right_click(world: &mut World) {
    let ui_state = *world.fetch::<UIState>();

    match ui_state {
        UIState::AddingWire {
            node_entity,
            wire_entity,
            x_pos: None,
            y_pos: Some(y_pos),
        } => {
            let (mx, _) = mouse_position();
            world.insert(UIState::AddingWire {
                node_entity,
                wire_entity,
                x_pos: Some(mx),
                y_pos: Some(y_pos),
            });
            world
                .write_storage::<Pos>()
                .insert(wire_entity, Pos::from_vec(Vec2::new(mx, y_pos)))
                .unwrap();
        }
        _ => {
            macro_rules! place_wire_systems {
                ( $([$node:ident, $i:expr, $o:expr]),* $(,)? ) => {
                    let initial_state = matches!(*world.fetch::<UIState>(), UIState::AddingWire{..});

                    $(
                        WirePlaceSys::<nodes::$node, $i, $o>::default().run_now(&world);
                        let new_state = matches!(*world.fetch::<UIState>(), UIState::AddingWire{..});
                        if new_state != initial_state {
                            return
                        }
                    )*
                };
            }

            use crate::all_nodes;
            all_nodes!(place_wire_systems);
        }
    }
}
