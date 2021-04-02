use crate::components::*;
use crate::components::{nodes::AndNode, Node};
use crate::nodes::OnNode;
use core::marker::PhantomData;
use macroquad::prelude::*;
use specs::prelude::*;
use std::sync::Arc;

pub struct DrawNodeSys<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    node: PhantomData<N>,
    draw_fn: Arc<dyn Fn(Pos)>,
}

impl<'a, N, const I: usize, const O: usize> System<'a> for DrawNodeSys<N, I, O>
where
    N: Node<I, O>,
{
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Connected<N, I, O>>,
        ReadStorage<'a, Wire>,
    );

    fn run(&mut self, (positions, nodes, wires): Self::SystemData) {
        (&positions, &nodes).join().for_each(|(self_pos, node)| {
            let pos = self_pos.pos;
            node.inputs.iter().filter_map(|o| o.as_ref()).for_each(|e| {
                let Pos { pos: wire_pos, .. } = positions.get(*e).unwrap();
                let wire = wires.get(*e).unwrap();
                draw_line(
                    pos.x,
                    pos.y,
                    wire_pos.x,
                    wire_pos.y,
                    5.0,
                    if wire.output_state { RED } else { WHITE },
                );
            });
            node.outputs
                .iter()
                .filter_map(|o| o.as_ref())
                .for_each(|e| {
                    let Pos { pos: wire_pos, .. } = positions.get(*e).unwrap();
                    let wire = wires.get(*e).unwrap();
                    draw_line(
                        pos.x,
                        pos.y,
                        wire_pos.x,
                        wire_pos.y,
                        5.0,
                        if wire.output_state { RED } else { WHITE },
                    );
                });
            (self.draw_fn)(*self_pos);
        });
    }
}

pub fn add_draw_system<'a, 'b>(builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    builder
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<OnNode>,
            draw_fn: Arc::new(|Pos { pos, .. }| {
                draw_rectangle(pos.x - 25.0, pos.y - 25.0, 50.0, 50.0, WHITE);
                draw_text("ON", pos.x - 12.5, pos.y, 25.0, BLACK);
            }),
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<AndNode>,
            draw_fn: Arc::new(|Pos { pos, .. }| {
                draw_rectangle(pos.x - 25.0, pos.y - 25.0, 50.0, 50.0, WHITE);
                draw_text("AND", pos.x - 12.5, pos.y, 25.0, BLACK);
            }),
        })
}
