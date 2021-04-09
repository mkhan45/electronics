use crate::Connected;
use crate::{components::Connection, nodes::Wire};
use crate::{components::Node, resources::Tick};
use core::marker::PhantomData;
use specs::prelude::*;

pub struct ElectroSys<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    node: PhantomData<N>,
}

impl<'a, N, const I: usize, const O: usize> Default for ElectroSys<N, I, O>
where
    N: Node<I, O> + 'static,
{
    fn default() -> Self {
        ElectroSys {
            node: PhantomData::<N>,
        }
    }
}

impl<'a, N, const I: usize, const O: usize> System<'a> for ElectroSys<N, I, O>
where
    N: Node<I, O> + 'static,
{
    type SystemData = (
        WriteStorage<'a, Connected<N, I, O>>,
        ReadStorage<'a, Connection>,
        WriteStorage<'a, Wire>,
    );

    fn run(&mut self, (mut nodes, connections, mut wires): Self::SystemData) {
        // direct inputs and outputs must be wires
        (&mut nodes).join().for_each(|node| {
            let mut inputs = [false; I];
            for (i, input_entity) in node.inputs.iter().enumerate() {
                let connection = connections.get(*input_entity).unwrap();
                match connection.wire {
                    Some(e) => {
                        let wire = wires.get(e).expect("All inputs must be a wire");
                        inputs[i] = wire.output_state;
                    }
                    None => return,
                }
            }

            let outputs = node.calculate_state(inputs);

            for (i, output_entity) in node.outputs.iter().enumerate() {
                let connection = connections.get(*output_entity).unwrap();
                if let Some(e) = connection.wire {
                    let wire = wires.get_mut(e).unwrap();
                    if wire.input_state != outputs[i] {
                        wire.input_state = outputs[i];
                        wire.changed_input = true;
                    } else {
                        wire.changed_input = false;
                    }
                }
            }
        })
    }
}

pub struct WireSys;
impl<'a> System<'a> for WireSys {
    type SystemData = WriteStorage<'a, Wire>;

    fn run(&mut self, mut wires: Self::SystemData) {
        (&mut wires).join().for_each(|wire| {
            wire.output_state = wire.input_state;
        });
    }
}

pub struct ResetSys;
impl<'a> System<'a> for ResetSys {
    type SystemData = (WriteStorage<'a, Wire>, Write<'a, Tick>);

    fn run(&mut self, (mut wires, mut tick): Self::SystemData) {
        (&mut wires).join().for_each(|wire| {
            wire.input_state = false;
            wire.output_state = false;
        });
        tick.0 = 0;
    }
}
