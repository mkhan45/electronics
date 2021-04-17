use crate::components::{Connection, ConnectionTy};
use crate::components::{CurrentScope, SNAP};
use crate::resources::UIState;
use crate::CompoundNodeData;
use crate::Wire;
use crate::{components::Pos, resources::MousePos};
use crate::{
    components::{round_to_snap, InnerNode},
    resources::CreatingCompoundNode,
};
use macroquad::prelude::Vec2;
use specs::prelude::*;

pub struct WirePlaceSys;
impl<'a> System<'a> for WirePlaceSys {
    type SystemData = (
        WriteStorage<'a, Connection>,
        WriteStorage<'a, Wire>,
        WriteStorage<'a, InnerNode>,
        WriteStorage<'a, CurrentScope>,
        ReadStorage<'a, Pos>,
        Write<'a, UIState>,
        Read<'a, MousePos>,
        Read<'a, CreatingCompoundNode>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut connections,
            mut wires,
            mut inner_node_data,
            mut current_scope_markers,
            positions,
            mut ui_state,
            mouse_pos,
            creating_compound,
            entities,
        ): Self::SystemData,
    ) {
        let mp = mouse_pos.0;

        match &*ui_state {
            UIState::Nothing => {
                // if a connection output is clicked, initialize wire adding
                let clicked_output = (
                    &mut connections,
                    &positions,
                    &entities,
                    &current_scope_markers,
                )
                    .join()
                    .find(|(conn, Pos { pos, .. }, _, _)| {
                        conn.ty == ConnectionTy::Output && (*pos - mp).length() < 7.5
                    });

                if let Some((_, Pos { pos, .. }, connection_entity, _)) = clicked_output {
                    *ui_state = UIState::AddingWire {
                        connection_entity,
                        points: vec![*pos],
                    };
                }
            }
            UIState::AddingWire { .. } => {
                let clicked_input = (
                    &mut connections,
                    &positions,
                    &entities,
                    &current_scope_markers,
                )
                    .join()
                    .find(|(conn, Pos { pos, .. }, _, _)| {
                        conn.ty == ConnectionTy::Input && (*pos - mp).length() < 7.5
                    });

                match &*ui_state {
                    UIState::AddingWire {
                        points,
                        connection_entity,
                    } if clicked_input.is_some() => {
                        let (clicked_connection, clicked_pos, _, _) = clicked_input.unwrap();

                        let mut start_point = positions.get(*connection_entity).unwrap().pos;
                        let end_point = clicked_pos.pos;

                        let mut points = points.to_vec();
                        points.push(end_point);

                        if let Some(last) = points.last_mut() {
                            if (last.y - end_point.y).abs() < SNAP / 2.0 {
                                last.y = end_point.y;
                            }
                        } else {
                            if (start_point.y - end_point.y).abs() < SNAP / 2.0 {
                                start_point.y = end_point.y;
                            }
                        }

                        // create the wire
                        let wire_entity = {
                            let mut builder = entities
                                .build_entity()
                                .with(CurrentScope, &mut current_scope_markers)
                                .with(
                                    Wire {
                                        start_point,
                                        end_point,
                                        points,
                                        ..Wire::default()
                                    },
                                    &mut wires,
                                );
                            if let Some(CompoundNodeData { entity, .. }) = creating_compound.0 {
                                builder = builder
                                    .with(InnerNode { parent: entity }, &mut inner_node_data);
                            }
                            builder.build()
                        };

                        // update the connections
                        clicked_connection.wires.push(wire_entity);
                        let fst_conn = connections.get_mut(*connection_entity).unwrap();
                        fst_conn.wires.push(wire_entity);

                        *ui_state = UIState::Nothing;
                    }
                    UIState::AddingWire {
                        points,
                        connection_entity,
                    } if clicked_input.is_none() => {
                        let mut points = points.clone();
                        points.push(Vec2::new(round_to_snap(mp.x), round_to_snap(mp.y)));
                        *ui_state = UIState::AddingWire {
                            points,
                            connection_entity: *connection_entity,
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => {}
        }
    }
}
