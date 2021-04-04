use crate::Pos;
use crate::{components::nodes::NandNode, nodes::NotNode};
use crate::{components::nodes::NorNode, nodes::OnNode};
use crate::{components::nodes::XnorNode, nodes::XorNode};
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
                        let delta = ((tick_progress.0 - 0.5) * 2.0).clamp(0.0, 1.0) as f32;

                        let horizontal_dist = ep.x - sp.x;
                        let vert_dist = ep.y - sp.y;
                        let total_dist = horizontal_dist.abs() + vert_dist.abs();
                        let red_dist = delta * total_dist;

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

                            draw_circle(
                                sp.x,
                                ep.y,
                                5.0,
                                // 0.1 is a magic number but it seems to work pretty well
                                if wire.input_state && delta >= 0.5 {
                                    RED
                                } else {
                                    WHITE
                                },
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

                        draw_circle(sp.x, ep.y, 5.0, if wire.input_state { RED } else { WHITE });
                    }
                });

            node.outputs
                .iter()
                .filter_map(|o| o.as_ref())
                .for_each(|e| {
                    if let Some(Pos { pos: wire_pos, .. }) = positions.get(*e) {
                        let wire = wires.get(*e).unwrap();

                        let sp = Vec2::new(pos.x, pos.y);
                        let ep = Vec2::new(wire_pos.x, pos.y);

                        if wire.changed_input {
                            let delta = (tick_progress.0 * 2.0).clamp(0.0, 1.0) as f32;
                            let diff = (ep - sp) * delta;
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

                            draw_circle(
                                ep.x,
                                sp.y,
                                5.0,
                                // 0.9 is a magic number but it seems to work pretty well
                                if wire.input_state && delta >= 0.9 {
                                    RED
                                } else {
                                    WHITE
                                },
                            );
                        } else {
                            draw_line(
                                sp.x,
                                sp.y,
                                ep.x,
                                sp.y,
                                5.0,
                                if wire.input_state { RED } else { WHITE },
                            );

                            draw_circle(
                                ep.x,
                                sp.y,
                                5.0,
                                if wire.input_state { RED } else { WHITE },
                            );
                        }
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
                draw_circle(pos.x, pos.y, 25.0, RED);
            }),
            input_offsets: [],
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<OffNode>,
            draw_fn: Arc::new(|Pos { pos, .. }, _| {
                draw_circle(pos.x, pos.y, 25.0, WHITE);
            }),
            input_offsets: [],
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<NotNode>,
            draw_fn: Arc::new(|Pos { pos, .. }, textures: &Textures| {
                let texture = textures.0.get("NOT_GATE").unwrap();
                let w = 50.0;
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
            input_offsets: [Vec2::new(-25.0, 0.0)],
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
            draw_fn: Arc::new(|Pos { pos, .. }, textures: &Textures| {
                let texture = textures.0.get("OR_GATE").unwrap();
                let w = 100.0;
                let h = 75.0;
                draw_texture_ex(
                    *texture,
                    pos.x - w / 2.0 + w * 0.1,
                    pos.y - h / 2.0 + w * 0.08,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(w * 0.8, h * 0.8)),
                        ..DrawTextureParams::default()
                    },
                );
            }),
            input_offsets: [Vec2::new(-25.0, -10.0), Vec2::new(-25.0, 10.0)],
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<NandNode>,
            draw_fn: Arc::new(|Pos { pos, .. }, textures: &Textures| {
                let texture = textures.0.get("NAND_GATE").unwrap();
                let w = 100.0;
                let h = 75.0;
                draw_texture_ex(
                    *texture,
                    pos.x - w / 2.0 + w * 0.1,
                    pos.y - h / 2.0 + w * 0.08,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(w * 0.8, h * 0.8)),
                        ..DrawTextureParams::default()
                    },
                );
            }),
            input_offsets: [Vec2::new(-25.0, -10.0), Vec2::new(-25.0, 10.0)],
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<NorNode>,
            draw_fn: Arc::new(|Pos { pos, .. }, textures: &Textures| {
                let texture = textures.0.get("NOR_GATE").unwrap();
                let w = 100.0;
                let h = 75.0;
                draw_texture_ex(
                    *texture,
                    pos.x - w / 2.0 + w * 0.1,
                    pos.y - h / 2.0 + w * 0.08,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(w * 0.8, h * 0.8)),
                        ..DrawTextureParams::default()
                    },
                );
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
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<XnorNode>,
            draw_fn: Arc::new(|Pos { pos, .. }, textures: &Textures| {
                let texture = textures.0.get("XNOR_GATE").unwrap();
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
