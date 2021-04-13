use crate::components::round_to_snap;
use crate::components::SNAP;
use crate::components::{Connection, ConnectionTy};
use crate::resources::UIState;
use crate::Wire;
use crate::{components::Pos, resources::MousePos};
use macroquad::prelude::Vec2;
use specs::prelude::*;

pub struct WirePlaceSys;
impl<'a> System<'a> for WirePlaceSys {
    type SystemData = (
        WriteStorage<'a, Connection>,
        WriteStorage<'a, Wire>,
        ReadStorage<'a, Pos>,
        Write<'a, UIState>,
        Read<'a, MousePos>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (mut connections, mut wires, positions, mut ui_state, mouse_pos, entities): Self::SystemData,
    ) {
        let mp = mouse_pos.0;

        let clicked = (&mut connections, &positions, &entities)
            .join()
            .find(|(_, Pos { pos, .. }, _)| (*pos - mp).length() < 7.5);

        match &*ui_state {
            UIState::Nothing => {
                // if a connection output is clicked, initialize wire adding
                if let Some((_, Pos { pos, .. }, connection_entity)) = clicked {
                    *ui_state = UIState::AddingWire {
                        connection_entity,
                        points: vec![*pos],
                    };
                }
            }
            UIState::AddingWire {
                points,
                connection_entity,
            } if clicked.is_some() => {
                let (clicked_connection, clicked_pos, _) = clicked.unwrap();

                let mut start_point = positions.get(*connection_entity).unwrap().pos;
                let end_point = clicked_pos.pos;

                let mut points = points.to_vec();
                if let Some(last) = points.last_mut() {
                    if (last.y - end_point.y).abs() < SNAP {
                        last.y = end_point.y;
                    }
                } else {
                    if (start_point.y - end_point.y).abs() < SNAP {
                        start_point.y = end_point.y;
                    }
                }

                // create the wire
                let wire_entity = entities
                    .build_entity()
                    .with(
                        Wire {
                            start_point,
                            end_point,
                            points,
                            ..Wire::default()
                        },
                        &mut wires,
                    )
                    .build();

                // update the connections
                clicked_connection.wires.push(wire_entity);
                let fst_conn = connections.get_mut(*connection_entity).unwrap();
                fst_conn.wires.push(wire_entity);

                *ui_state = UIState::Nothing;
            }
            UIState::AddingWire {
                points,
                connection_entity,
            } if clicked.is_none() => {
                let mut points = points.clone();
                points.push(Vec2::new(round_to_snap(mp.x), round_to_snap(mp.y)));
                *ui_state = UIState::AddingWire {
                    points,
                    connection_entity: *connection_entity,
                }
            }
            _ => {}
        }
    }
}
