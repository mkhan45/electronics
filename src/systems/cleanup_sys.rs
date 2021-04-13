use std::marker::PhantomData;

use crate::components::nodes::*;
use crate::components::Node;
use crate::{components::Connection, Connected};
use specs::prelude::*;

#[derive(Default)]
pub struct CleanupWires;
impl<'a> System<'a> for CleanupWires {
    type SystemData = (
        WriteStorage<'a, Connection>,
        ReadStorage<'a, Wire>,
        Entities<'a>,
    );

    fn run(&mut self, (mut connections, wires, entities): Self::SystemData) {
        (&mut connections).join().for_each(|connection| {
            connection.wires = connection
                .wires
                .iter()
                .filter(|e| entities.is_alive(**e))
                .copied()
                .collect();
        });
    }
}

pub struct CleanupConnectionSys<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    node: PhantomData<N>,
    entity: Entity,
}

impl<'a, N, const I: usize, const O: usize> System<'a> for CleanupConnectionSys<N, I, O>
where
    N: Node<I, O> + 'static,
{
    type SystemData = (
        ReadStorage<'a, Connected<N, I, O>>,
        ReadStorage<'a, Connection>,
        Entities<'a>,
    );

    fn run(&mut self, (nodes, connections, entities): Self::SystemData) {
        if let Some(node) = nodes.get(self.entity) {
            node.inputs
                .iter()
                .chain(node.outputs.iter())
                .for_each(|connection_entity| {
                    let connection = connections.get(*connection_entity).unwrap();
                    connection.wires.iter().for_each(|wire| {
                        entities.delete(*wire).unwrap();
                    });
                    entities.delete(*connection_entity).unwrap();
                });
        }
    }
}

pub fn run_cleanup_systems(entity: Entity, world: &World) {
    use crate::all_nodes;

    macro_rules! run_cleanup_sys {
        ( $([$node:ident, $i:expr, $o:expr]),* $(,)? ) => {
            $(
                CleanupConnectionSys {
                    node: PhantomData::<$node>,
                    entity,
                }.run_now(world);
            )*
        };
    }

    all_nodes!(run_cleanup_sys);
}
