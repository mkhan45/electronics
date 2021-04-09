use std::marker::PhantomData;

use crate::components::nodes::*;
use crate::components::Node;
use crate::{components::Connection, Connected};
use specs::prelude::*;

#[derive(Default)]
pub struct CleanupWires;
impl<'a> System<'a> for CleanupWires {
    type SystemData = (WriteStorage<'a, Connection>, Entities<'a>);

    fn run(&mut self, (mut connections, entities): Self::SystemData) {
        (&mut connections).join().for_each(|connection| {
            if let Some(wire_e) = connection.wire {
                if !entities.is_alive(wire_e) {
                    connection.wire = None;
                }
            }
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
    type SystemData = (ReadStorage<'a, Connected<N, I, O>>, Entities<'a>);

    fn run(&mut self, (nodes, entities): Self::SystemData) {
        if let Some(node) = nodes.get(self.entity) {
            node.inputs
                .iter()
                .for_each(|connection| entities.delete(*connection).unwrap());
            node.outputs
                .iter()
                .for_each(|connection| entities.delete(*connection).unwrap());
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
