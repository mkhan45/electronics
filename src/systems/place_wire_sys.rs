use crate::components::{Connected, Node, Orientation, Pos, Wire};
use crate::resources::AddingWire;
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
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Wire>,
        Write<'a, AddingWire>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (mut nodes, mut positions, mut wires, mut adding_wire, entities): Self::SystemData,
    ) {
        let (mx, my) = mouse_position();
        let mp = Vec2::new(mx, my);

        let filtered = (&mut nodes, &positions, &entities)
            .join()
            .filter(|(_, pos, _)| (pos.pos - mp).length() < 35.0);

        match adding_wire.0 {
            Some((wire_entity, input_i)) => {
                for (node, pos, _) in filtered {
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
                        *adding_wire = AddingWire(None);
                        break;
                    }
                }
            }
            None => {
                for (node, pos, _) in filtered {
                    // current node is potential wire input
                    let mut first_empty = node.outputs.iter().enumerate().find_map(|(i, o)| {
                        if o.is_none() {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    if first_empty.is_none() && O == 1 {
                        first_empty = Some(0);
                    }

                    if let Some(i) = first_empty {
                        let wire_entity = entities
                            .build_entity()
                            .with(Wire::default(), &mut wires)
                            .build();
                        node.outputs[i] = Some(wire_entity);
                        *adding_wire = AddingWire(Some((wire_entity, None)));
                        break;
                    }
                }
            }
        }
    }
}
