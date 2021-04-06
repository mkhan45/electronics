use crate::Connected;
use std::marker::PhantomData;

use crate::components::Node;
use specs::prelude::*;

#[derive(Default)]
pub struct CleanupWires<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    node: PhantomData<N>,
}
impl<'a, N, const I: usize, const O: usize> System<'a> for CleanupWires<N, I, O>
where
    N: Node<I, O> + 'static,
{
    type SystemData = (WriteStorage<'a, Connected<N, I, O>>, Entities<'a>);

    fn run(&mut self, (mut nodes, entities): Self::SystemData) {
        (&mut nodes).join().for_each(|node| {
            node.inputs
                .iter_mut()
                .chain(node.outputs.iter_mut())
                .filter(|wire_opt| wire_opt.is_some())
                .for_each(|o: &mut Option<Entity>| {
                    let wire_e = o.unwrap();
                    dbg!(wire_e);
                    if !entities.is_alive(wire_e) {
                        *o = None;
                    }
                });
        });
    }
}

pub fn run_cleanup_sys(world: &mut World) {
    use crate::nodes::*;

    macro_rules! cleanup_nodes {
        ( $([$node:ident, $i:expr, $o:expr]),*, $(,)? ) => {
            $(CleanupWires::<$node, $i, $o>::default().run_now(world);)*
        };
    }

    use crate::all_nodes;
    all_nodes!(cleanup_nodes);
}
