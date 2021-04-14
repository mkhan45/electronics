use crate::components::Connection;
use crate::components::Pos;
use crate::resources::MousePos;
use crate::resources::UIState;
use crate::systems::place_node_sys::PlaceNodeSys;
use specs::prelude::*;

use crate::nodes;
pub fn handle_mouse_click(world: &mut World) {
    crate::systems::ui_systems::SwitchClickSys.run_now(world);

    let mut ui_state = world.fetch_mut::<UIState>();

    match *ui_state {
        UIState::AddingWire { .. } => {
            *ui_state = UIState::Nothing;
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

            *ui_state = UIState::Nothing;
        }
        UIState::Deleting => {
            let positions = world.read_storage::<Pos>();
            let entities = world.entities();
            let mouse_pos = world.fetch::<MousePos>().0;
            let connections = world.read_storage::<Connection>();
            let target = (&positions, &entities).join().find(|(pos, e)| {
                (connections.get(*e).is_none()) && (pos.pos - mouse_pos).length() < 35.0
            });
            std::mem::drop(connections);

            if let Some((_, entity)) = target {
                entities.delete(entity).unwrap();
                std::mem::drop(positions);
                std::mem::drop(entities);
                std::mem::drop(ui_state);
                crate::systems::cleanup_sys::run_cleanup_systems(entity, world);
                world.maintain();
                crate::systems::cleanup_sys::CleanupWires.run_now(world);
            }
        }
        UIState::Nothing => {}
    }
}

pub fn handle_mouse_right_click(world: &mut World) {
    // let ui_state = *world.fetch::<UIState>();

    crate::systems::place_wire_sys::WirePlaceSys.run_now(world);
    // match ui_state {
    // UIState::AddingWire {
    //     connection_entity,
    //     wire_entity,
    //     x_pos: None,
    //     y_pos: Some(y_pos),
    // } => {
    //     let mp = world.fetch::<MousePos>().0;
    //     let mut connections = world.write_storage::<Connection>();
    //     let positions = world.read_storage::<Pos>();
    //     let clicked = (&mut connections, &positions).join().find_map(|(c, pos)| {
    //         if (pos.pos - mp).length() < 10.0 && c.ty == ConnectionTy::Input {
    //             Some(c)
    //         } else {
    //             None
    //         }
    //     });
    //     std::mem::drop(positions);

    //     if let Some(connection) = clicked {
    //         connection.wire = Some(wire_entity);
    //         std::mem::drop(connections);

    //         {
    //             let mut wires = world.write_storage::<Wire>();
    //             let wire = wires.get_mut(wire_entity).unwrap();
    //             wire.points
    //                 .push(Vec2::new(round_to_snap(mp.x) - SNAP, y_pos));
    //         }

    //         world.insert(UIState::Nothing);
    //     } else {
    //         std::mem::drop(connections);
    //         let mouse_pos = world.fetch::<MousePos>().0;
    //         world.insert(UIState::AddingWire {
    //             connection_entity,
    //             wire_entity,
    //             x_pos: Some(mouse_pos.x),
    //             y_pos: Some(y_pos),
    //         });

    //         let mut wires = world.write_storage::<Wire>();
    //         let wire = wires.get_mut(wire_entity).unwrap();

    //         wire.points
    //             .push(Vec2::new(round_to_snap(mouse_pos.x), round_to_snap(y_pos)));
    //     }
    // }
    // _ => {
    // }
    // }
}
