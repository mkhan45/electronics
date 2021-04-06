use crate::systems::place_node_sys::PlaceNodeSys;
use crate::systems::place_wire_sys::WirePlaceSys;
use crate::{components::Pos, resources::AddingWire};
use macroquad::prelude::*;
use specs::prelude::*;

use crate::nodes;
use crate::resources;

pub fn handle_mouse_click(world: &mut World) {
    // clear adding wire including removing the wire entity
    {
        let adding_wire_state = world.fetch::<AddingWire>().0;
        if let Some((_, wire_entity, _, _)) = adding_wire_state {
            dbg!(wire_entity);
            world.delete_entity(wire_entity).unwrap();

            crate::systems::cleanup_sys::run_cleanup_sys(world);

            std::mem::drop(adding_wire_state);
            world.insert(AddingWire(None));
        }
    }
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

        use crate::all_nodes;
        all_nodes!(place_node_systems);

        std::mem::drop(node);
        world.insert(resources::AddingNode(None));
    }
}

pub fn handle_mouse_right_click(world: &mut World) {
    let adding_wire = world.fetch::<AddingWire>().0;
    match adding_wire {
        Some((node_entity, wire_entity, None, Some(y_pos))) => {
            let (mx, _) = mouse_position();
            world.insert(AddingWire(Some((
                node_entity,
                wire_entity,
                Some(mx),
                Some(y_pos),
            ))));
            world
                .write_storage::<Pos>()
                .insert(wire_entity, Pos::from_vec(Vec2::new(mx, y_pos)))
                .unwrap();
        }
        _ => {
            macro_rules! place_wire_systems {
                ( $([$node:ident, $i:expr, $o:expr]),* $(,)? ) => {
                    let initial_state = world.fetch::<AddingWire>().0.is_some();

                    $(
                        WirePlaceSys::<nodes::$node, $i, $o>::default().run_now(&world);
                        let new_state = world.fetch::<AddingWire>().0.is_some();
                        if new_state != initial_state {
                            return
                        }
                    )*
                };
            }

            use crate::all_nodes;
            all_nodes!(place_wire_systems);
        }
    };
}
