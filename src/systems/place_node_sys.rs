use crate::{
    components::{Connection, ConnectionTy, CurrentScope, InnerNode, Node, NodeMarker},
    resources::CreatingCompoundNode,
};
use crate::{resources::CompoundNodeData, Pos};
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
        WriteStorage<'a, InnerNode>,
        Read<'a, MousePos>,
        Read<'a, CreatingCompoundNode>,
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
            mut inner_nodes,
            mouse_pos,
            creating_compound,
            entities,
        ): Self::SystemData,
    ) {
        let pos = Pos::from_vec(mouse_pos.0);
        let input_offsets = N::input_offsets();
        let output_offsets = N::output_offsets();

        // I wanted this to be a closure but ownership pain
        // that's probably a code smell
        macro_rules! add_inner_node_data {
            ( $builder:expr ) => {
                if let Some(CompoundNodeData { entity, .. }) = creating_compound.0 {
                    $builder = $builder.with(InnerNode { parent: entity }, &mut inner_nodes);
                }
            };
        }

        let inputs = (0..I)
            .map(|index| {
                let mut builder = entities
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
                    );
                add_inner_node_data!(builder);
                // if let Some(CompoundNodeData { entity, .. }) = creating_compound.0 {
                //     builder = builder.with(InnerNode { parent: entity }, &mut inner_nodes);
                // }
                builder.build()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let outputs = (0..O)
            .map(|index| {
                let mut builder = entities
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
                    );
                add_inner_node_data!(builder);
                builder.build()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let mut builder = entities
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
            .with(pos, &mut position_storage);
        add_inner_node_data!(builder);
        builder.build();
    }
}
