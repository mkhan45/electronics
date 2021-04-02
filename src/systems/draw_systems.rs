use crate::components::{nodes::AndNode, Node};
use crate::nodes::OnNode;
use crate::Pos;
use crate::{
    components::nodes::{OffNode, OrNode},
    resources::TickProgress,
};
use crate::{components::Wire, Connected};
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
        Read<'a, TickProgress>,
    );

    fn run(&mut self, (positions, nodes, wires, tick_progress): Self::SystemData) {
        (&positions, &nodes).join().for_each(|(self_pos, node)| {
            let pos = self_pos.pos;
            node.inputs.iter().filter_map(|o| o.as_ref()).for_each(|e| {
                let Pos { pos: wire_pos, .. } = positions.get(*e).unwrap();
                let wire = wires.get(*e).unwrap();

                let sp = *wire_pos;
                let ep = Vec2::new(pos.x, pos.y);

                if wire.output_state != wire.input_state {
                    let diff = (ep - sp) * ((tick_progress.0 - 0.5) * 2.0).clamp(0.0, 1.0) as f32;
                    let mid = sp + diff;

                    draw_line(
                        sp.x,
                        sp.y,
                        mid.x,
                        mid.y,
                        5.0,
                        if wire.input_state { RED } else { WHITE },
                    );

                    if mid != ep {
                        draw_line(
                            mid.x,
                            mid.y,
                            ep.x,
                            ep.y,
                            5.0,
                            if wire.input_state { WHITE } else { RED },
                        );
                    }
                } else {
                    draw_line(
                        sp.x,
                        sp.y,
                        ep.x,
                        ep.y,
                        5.0,
                        if wire.input_state { RED } else { WHITE },
                    );
                }
            });

            node.outputs
                .iter()
                .filter_map(|o| o.as_ref())
                .for_each(|e| {
                    let Pos { pos: wire_pos, .. } = positions.get(*e).unwrap();
                    let wire = wires.get(*e).unwrap();

                    let sp = Vec2::new(pos.x, pos.y);
                    let ep = *wire_pos;

                    if wire.changed_input {
                        let diff = (ep - sp) * (tick_progress.0 * 2.0).clamp(0.0, 1.0) as f32;
                        let mid = sp + diff;

                        draw_line(
                            sp.x,
                            sp.y,
                            mid.x,
                            mid.y,
                            5.0,
                            if wire.input_state { RED } else { WHITE },
                        );

                        if mid != ep {
                            draw_line(
                                mid.x,
                                mid.y,
                                ep.x,
                                ep.y,
                                5.0,
                                if wire.input_state { WHITE } else { RED },
                            );
                        }
                    } else {
                        draw_line(
                            sp.x,
                            sp.y,
                            ep.x,
                            ep.y,
                            5.0,
                            if wire.input_state { RED } else { WHITE },
                        );
                    }
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
            node: PhantomData::<OffNode>,
            draw_fn: Arc::new(|Pos { pos, .. }| {
                draw_rectangle(pos.x - 25.0, pos.y - 25.0, 50.0, 50.0, WHITE);
                draw_text("OFF", pos.x - 12.5, pos.y, 25.0, BLACK);
            }),
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<AndNode>,
            draw_fn: Arc::new(|Pos { pos, .. }| {
                draw_rectangle(pos.x - 25.0, pos.y - 25.0, 50.0, 50.0, WHITE);
                draw_text("AND", pos.x - 12.5, pos.y, 25.0, BLACK);
            }),
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<OrNode>,
            draw_fn: Arc::new(|Pos { pos, .. }| {
                draw_rectangle(pos.x - 25.0, pos.y - 25.0, 50.0, 50.0, WHITE);
                draw_text("OR", pos.x - 12.5, pos.y, 25.0, BLACK);
            }),
        })
}
