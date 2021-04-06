use crate::components::{Connected, Node, Pos};
use crate::Wire;
use crate::{components::round_to_snap, resources::UIState};
use core::marker::PhantomData;
use macroquad::prelude::*;
use specs::prelude::*;

#[derive(Default)]
pub struct WirePlaceSys<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    node: PhantomData<N>,
}

impl<'a, N, const I: usize, const O: usize> System<'a> for WirePlaceSys<N, I, O>
where
    N: Node<I, O> + 'static,
{
    type SystemData = (
        WriteStorage<'a, Connected<N, I, O>>,
        ReadStorage<'a, Pos>,
        WriteStorage<'a, Wire>,
        Write<'a, UIState>,
        Entities<'a>,
    );

    fn run(&mut self, (mut nodes, positions, mut wires, mut ui_state, entities): Self::SystemData) {
        let (mx, my) = mouse_position();
        let mp = Vec2::new(round_to_snap(mx), round_to_snap(my));

        let filtered = (&mut nodes, &positions, &entities)
            .join()
            .filter(|(_, pos, _)| (pos.pos - mp).length() < 35.0);

        match *ui_state {
            UIState::AddingWire { wire_entity, .. } => {
                for (node, _, _) in filtered {
                    // current node is potential wire output
                    let first_empty = node.inputs.iter().enumerate().find_map(|(i, o)| {
                        if o.is_none() {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    if let Some(i) = first_empty {
                        node.inputs[i] = Some(wire_entity);
                        *ui_state = UIState::Nothing;
                        break;
                    }
                }
            }
            _ => {
                for (node, Pos { pos, .. }, node_entity) in filtered {
                    // current node is potential wire input
                    let first_empty = node.outputs.iter().enumerate().find_map(|(i, o)| {
                        if o.is_none() {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    if first_empty.is_none() && O == 1 {
                        let wire_entity = node.outputs[0].unwrap();
                        let wire_pos = positions.get(wire_entity).unwrap();
                        *ui_state = UIState::AddingWire {
                            node_entity,
                            wire_entity,
                            x_pos: Some(wire_pos.pos.x),
                            y_pos: Some(pos.y),
                        };
                    } else {
                        if let Some(i) = first_empty {
                            let wire_entity = entities
                                .build_entity()
                                .with(Wire::default(), &mut wires)
                                .build();
                            node.outputs[i] = Some(wire_entity);
                            *ui_state = UIState::AddingWire {
                                node_entity,
                                wire_entity,
                                x_pos: None,
                                y_pos: Some(pos.y),
                            };
                            break;
                        }
                    }
                }
            }
        };
    }
}
