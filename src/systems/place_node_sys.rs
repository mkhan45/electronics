use crate::components::{Connection, ConnectionTy, CurrentScope, Node, NodeMarker};
use crate::Pos;
use crate::{resources::MousePos, Connected};
use core::marker::PhantomData;
use specs::prelude::*;
use std::convert::TryInto;

#[derive(Default)]
pub struct PlaceNodeSys<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    node: PhantomData<N>,
}

impl<'a, N, const I: usize, const O: usize> System<'a> for PlaceNodeSys<N, I, O>
where
    N: Node<I, O> + 'static,
{
    type SystemData = (
        WriteStorage<'a, Connected<N, I, O>>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Connection>,
        WriteStorage<'a, NodeMarker>,
        WriteStorage<'a, CurrentScope>,
        Read<'a, MousePos>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut node_storage,
            mut position_storage,
            mut connections,
            mut node_markers,
            mut current_scope_markers,
            mouse_pos,
            entities,
        ): Self::SystemData,
    ) {
        let pos = Pos::from_vec(mouse_pos.0);
        let input_offsets = N::input_offsets();
        let output_offsets = N::output_offsets();

        let inputs = (0..I)
            .map(|index| {
                entities
                    .build_entity()
                    .with(CurrentScope, &mut current_scope_markers)
                    .with(
                        Connection {
                            wires: Vec::new(),
                            ty: ConnectionTy::Input,
                            index,
                        },
                        &mut connections,
                    )
                    .with(
                        Pos::from_vec_unrounded(pos.pos + input_offsets[index]),
                        &mut position_storage,
                    )
                    .build()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let outputs = (0..O)
            .map(|index| {
                entities
                    .build_entity()
                    .with(CurrentScope, &mut current_scope_markers)
                    .with(
                        Connection {
                            wires: Vec::new(),
                            ty: ConnectionTy::Output,
                            index,
                        },
                        &mut connections,
                    )
                    .with(
                        Pos::from_vec_unrounded(pos.pos + output_offsets[index]),
                        &mut position_storage,
                    )
                    .build()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        entities
            .build_entity()
            .with(NodeMarker, &mut node_markers)
            .with(CurrentScope, &mut current_scope_markers)
            .with(
                Connected {
                    node: N::default(),
                    inputs,
                    outputs,
                },
                &mut node_storage,
            )
            .with(pos, &mut position_storage)
            .build();
    }
}
