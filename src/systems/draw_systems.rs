use crate::nodes::OnNode;
use crate::nodes::XorNode;
use crate::Pos;
use crate::{
    components::nodes::{OffNode, OrNode},
    resources::TickProgress,
};
use crate::{components::Wire, Connected};
use crate::{
    components::{nodes::AndNode, Node},
    resources::Textures,
};
use core::marker::PhantomData;
use macroquad::prelude::*;
use specs::prelude::*;
use std::sync::Arc;

pub struct DrawNodeSys<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    node: PhantomData<N>,
    draw_fn: Arc<dyn Fn(Pos, &Textures)>,
    input_offsets: [Vec2; I],
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
        Read<'a, Textures>,
    );

    fn run(&mut self, (positions, nodes, wires, tick_progress, textures): Self::SystemData) {
        (&positions, &nodes).join().for_each(|(self_pos, node)| {
            let pos = self_pos.pos;
            node.inputs
                .iter()
                .enumerate()
                .filter_map(|(i, o)| o.map(|e| (i, e)))
                .for_each(|(i, e)| {
                    let Pos { pos: wire_pos, .. } = positions.get(e).unwrap();
                    let wire = wires.get(e).unwrap();

                    let sp = *wire_pos;
                    let ep = Vec2::new(pos.x, pos.y) + self.input_offsets[i];

                    if wire.output_state != wire.input_state {
                        // if false {
                        let diff = ((tick_progress.0 - 0.5) * 2.0).clamp(0.0, 1.0) as f32;

                        let horizontal_dist = ep.x - sp.x;
                        let vert_dist = ep.y - sp.y;
                        let total_dist = horizontal_dist.abs() + vert_dist.abs();
                        let red_dist = diff * total_dist;

                        // vertical
                        {
                            let midpoint = if red_dist < vert_dist.abs() {
                                sp.y + vert_dist.signum() * red_dist
                            } else {
                                sp.y + vert_dist
                            };

                            draw_line(
                                sp.x,
                                sp.y,
                                sp.x,
                                midpoint,
                                5.0,
                                if wire.input_state { RED } else { WHITE },
                            );

                            if vert_dist.abs() > red_dist {
                                draw_line(
                                    sp.x,
                                    midpoint,
                                    sp.x,
                                    ep.y,
                                    5.0,
                                    if wire.input_state { WHITE } else { RED },
                                )
                            }
                        }

                        // horizontal
                        {
                            let midpoint = sp.x + (red_dist - vert_dist.abs()).max(0.0);

                            draw_line(
                                sp.x,
                                ep.y,
                                midpoint,
                                ep.y,
                                5.0,
                                if wire.input_state { RED } else { WHITE },
                            );

                            draw_line(
                                midpoint,
                                ep.y,
                                ep.x,
                                ep.y,
                                5.0,
                                if wire.input_state { WHITE } else { RED },
                            );
                        }
                    } else {
                        // vertical
                        draw_line(
                            sp.x,
                            sp.y,
                            sp.x,
                            ep.y,
                            5.0,
                            if wire.input_state { RED } else { WHITE },
                        );

                        // horizontal
                        draw_line(
                            sp.x,
                            ep.y,
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
                            sp.y,
                            5.0,
                            if wire.input_state { RED } else { WHITE },
                        );
                    }
                });
            (self.draw_fn)(*self_pos, &textures);
        });
    }
}

pub fn add_draw_system<'a, 'b>(builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    builder
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<OnNode>,
            draw_fn: Arc::new(|Pos { pos, .. }, _| {
                draw_rectangle(pos.x - 25.0, pos.y - 25.0, 50.0, 50.0, WHITE);
                draw_text("ON", pos.x - 12.5, pos.y, 25.0, BLACK);
            }),
            input_offsets: [],
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<OffNode>,
            draw_fn: Arc::new(|Pos { pos, .. }, _| {
                draw_rectangle(pos.x - 25.0, pos.y - 25.0, 50.0, 50.0, WHITE);
                draw_text("OFF", pos.x - 12.5, pos.y, 25.0, BLACK);
            }),
            input_offsets: [],
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<AndNode>,
            draw_fn: Arc::new(|Pos { pos, .. }, textures: &Textures| {
                let texture = textures.0.get("AND_GATE").unwrap();
                let w = 75.0;
                let h = 50.0;
                draw_texture_ex(
                    *texture,
                    pos.x - w / 2.0,
                    pos.y - h / 2.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(w, h)),
                        ..DrawTextureParams::default()
                    },
                );
            }),
            input_offsets: [Vec2::new(-25.0, -10.0), Vec2::new(-25.0, 10.0)],
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<OrNode>,
            draw_fn: Arc::new(|Pos { pos, .. }, _: &Textures| {
                draw_rectangle(pos.x - 25.0, pos.y - 25.0, 50.0, 50.0, WHITE);
                draw_text("OR", pos.x - 12.5, pos.y, 25.0, BLACK);
            }),
            input_offsets: [Vec2::new(-25.0, -10.0), Vec2::new(-25.0, 10.0)],
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<XorNode>,
            draw_fn: Arc::new(|Pos { pos, .. }, textures: &Textures| {
                let texture = textures.0.get("XOR_GATE").unwrap();
                let w = 100.0;
                let h = 75.0;
                draw_texture_ex(
                    *texture,
                    pos.x - w / 2.0 + w * 0.1,
                    pos.y - h / 2.0 + w * 0.1,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(w * 0.8, h * 0.8)),
                        ..DrawTextureParams::default()
                    },
                );
            }),
            input_offsets: [Vec2::new(-25.0, -10.0), Vec2::new(-25.0, 10.0)],
        })
}
