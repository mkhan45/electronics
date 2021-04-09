use crate::Pos;
use crate::Wire;
use crate::{components::nodes::NandNode, nodes::NotNode};
use crate::{components::nodes::NorNode, nodes::OnNode};
use crate::{components::nodes::XnorNode, nodes::XorNode};
use crate::{
    components::nodes::{OffNode, OrNode},
    resources::TickProgress,
    resources::UIState,
};
use crate::{components::Connection, nodes::SwitchNode};
use crate::{
    components::{nodes::AndNode, Node},
    resources::Textures,
};
use crate::{resources::GridMode, Connected};
use core::marker::PhantomData;
use macroquad::prelude::*;
use specs::prelude::*;
use std::sync::Arc;

pub struct DrawNodeSys<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    node: PhantomData<N>,
    draw_fn: Arc<dyn Fn(&N, Pos, &Textures)>,
}

impl<'a, N, const I: usize, const O: usize> System<'a> for DrawNodeSys<N, I, O>
where
    N: Node<I, O>,
{
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Connected<N, I, O>>,
        ReadStorage<'a, Connection>,
        ReadStorage<'a, Wire>,
        Read<'a, TickProgress>,
        Read<'a, Textures>,
    );

    fn run(
        &mut self,
        (positions, nodes, connections, wires, tick_progress, textures): Self::SystemData,
    ) {
        let input_offsets = N::input_offsets();

        (&positions, &nodes).join().for_each(|(self_pos, node)| {
            let pos = self_pos.pos;
            node.inputs
                .iter()
                .enumerate()
                .filter_map(|(i, c)| connections.get(*c).unwrap().wire.map(|w| (i, w)))
                .for_each(|(i, e)| {
                    if let Some(Pos { pos: wire_pos, .. }) = positions.get(e) {
                        let wire = wires.get(e).unwrap();

                        let sp = *wire_pos;
                        let ep = Vec2::new(pos.x, pos.y) + input_offsets[i];

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
                                let midpoint = sp.x
                                    + horizontal_dist.signum()
                                        * (red_dist - vert_dist.abs()).max(0.0);

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

                                let (new_col, old_col) = if wire.input_state {
                                    (RED, WHITE)
                                } else {
                                    (WHITE, RED)
                                };

                                draw_circle(
                                    sp.x,
                                    ep.y,
                                    5.0,
                                    if delta >= vert_dist.abs() / total_dist {
                                        new_col
                                    } else {
                                        old_col
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

                            draw_circle(
                                sp.x,
                                ep.y,
                                5.0,
                                if wire.input_state { RED } else { WHITE },
                            );
                        }
                    }
                });

            node.outputs
                .iter()
                .filter_map(|c| connections.get(*c).unwrap().wire.as_ref())
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

                            let (new_col, old_col) = if wire.input_state {
                                (RED, WHITE)
                            } else {
                                (WHITE, RED)
                            };

                            draw_circle(
                                ep.x,
                                sp.y,
                                5.0,
                                if delta >= 0.9 { new_col } else { old_col },
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
            (self.draw_fn)(&node.node, *self_pos, &textures);
        });
    }
}

pub struct TempWireDrawSys;
impl<'a> System<'a> for TempWireDrawSys {
    type SystemData = (Read<'a, UIState>, ReadStorage<'a, Pos>);

    fn run(&mut self, (ui_state, position_storage): Self::SystemData) {
        use crate::components::round_to_snap as snap;
        let color = LIGHTGRAY;
        match *ui_state {
            UIState::AddingWire {
                connection_entity: e,
                x_pos: None,
                y_pos: Some(_),
                ..
            } => {
                let start_pos = Pos::from_vec(position_storage.get(e).unwrap().pos).pos;

                draw_line(
                    start_pos.x,
                    start_pos.y,
                    snap(mouse_position().0),
                    start_pos.y,
                    5.0,
                    color,
                );
            }
            UIState::AddingWire {
                x_pos: Some(x_pos),
                y_pos: Some(y_pos),
                ..
            } => {
                let (mx, my) = mouse_position();
                let pos = Pos::from_vec(Vec2::new(mx, my)).pos;

                draw_line(snap(x_pos), snap(y_pos), snap(x_pos), pos.y, 5.0, color);
                draw_line(snap(x_pos), pos.y, pos.x, pos.y, 5.0, color);
            }
            _ => {}
        }
    }
}

pub struct DrawConnectionSys;
impl<'a> System<'a> for DrawConnectionSys {
    type SystemData = (ReadStorage<'a, Connection>, ReadStorage<'a, Pos>);

    fn run(&mut self, (connections, positions): Self::SystemData) {
        let mouse_pos = {
            let (mx, my) = mouse_position();
            Vec2::new(mx, my)
        };

        let color = |pos: Vec2| {
            if (pos - mouse_pos).length() > 10.0 {
                Color::from_rgba(180, 180, 180, 215)
            } else {
                DARKGRAY
            }
        };

        (&connections, &positions)
            .join()
            .for_each(|(_, Pos { pos, .. })| draw_circle(pos.x, pos.y, 10.0, color(*pos)));
    }
}

pub struct DrawGridSys;
impl<'a> System<'a> for DrawGridSys {
    type SystemData = Read<'a, GridMode>;

    fn run(&mut self, grid_mode: Self::SystemData) {
        use crate::components::SNAP;
        let s = 4;

        match *grid_mode {
            GridMode::Lines => {
                // lines
                let base_width = 0.5;
                let wider_width = 1.5;

                (0..(screen_width() / SNAP).ceil() as usize)
                    .map(|i| (i, if i % s == 0 { wider_width } else { base_width }))
                    .map(|(i, width)| (i as f32 * SNAP, width))
                    .for_each(|(x, width)| draw_line(x, 0.0, x, screen_height(), width, DARKGRAY));

                (0..(screen_height() / SNAP).ceil() as usize)
                    .map(|i| (i, if i % s == 0 { wider_width } else { base_width }))
                    .map(|(i, width)| (i as f32 * SNAP, width))
                    .for_each(|(y, width)| draw_line(0.0, y, screen_width(), y, width, DARKGRAY));
            }
            GridMode::Dots => {
                let base_rad = 1.5;
                let wider_rad = 3.0;

                let x_positions = (0..(screen_width() / SNAP).ceil() as usize)
                    .map(|i| (i, i % s == 0))
                    .map(|(i, is_big)| (i as f32 * SNAP, is_big));
                let y_positions = (0..(screen_height() / SNAP).ceil() as usize)
                    .map(|i| (i, i % s == 0))
                    .map(|(i, is_big)| (i as f32 * SNAP, is_big));

                for (x, b1) in x_positions {
                    for (y, b2) in y_positions.clone() {
                        let rad = if b1 && b2 { wider_rad } else { base_rad };
                        draw_circle(x, y, rad, Color::from_rgba(155, 155, 155, 255));
                    }
                }
            }
            GridMode::CrossHatches => {
                let base_thickness = 0.75;
                let base_length = 8.0;
                let wider_thickness = 1.25;
                let wider_length = 15.0;

                let x_positions = (0..(screen_width() / SNAP).ceil() as usize)
                    .map(|i| (i, i % s == 0))
                    .map(|(i, is_big)| (i as f32 * SNAP, is_big));
                let y_positions = (0..(screen_height() / SNAP).ceil() as usize)
                    .map(|i| (i, i % s == 0))
                    .map(|(i, is_big)| (i as f32 * SNAP, is_big));

                for (x, b1) in x_positions {
                    for (y, b2) in y_positions.clone() {
                        let (thickness, len) = if b1 && b2 {
                            (wider_thickness, wider_length)
                        } else {
                            (base_thickness, base_length)
                        };

                        draw_line(x - len / 2.0, y, x + len / 2.0, y, thickness, DARKGRAY);
                        draw_line(x, y - len / 2.0, x, y + len / 2.0, thickness, DARKGRAY);
                    }
                }
            }
            GridMode::Off => {}
        }
    }
}

pub fn add_draw_system<'a, 'b>(builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    builder
        .with_thread_local(TempWireDrawSys)
        .with_thread_local(DrawGridSys)
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<OnNode>,
            draw_fn: Arc::new(|_, Pos { pos, .. }, _| {
                draw_circle(pos.x, pos.y, 25.0, RED);
            }),
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<OffNode>,
            draw_fn: Arc::new(|_, Pos { pos, .. }, _| {
                draw_circle(pos.x, pos.y, 25.0, WHITE);
            }),
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<NotNode>,
            draw_fn: Arc::new(|_, Pos { pos, .. }, textures: &Textures| {
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
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<AndNode>,
            draw_fn: Arc::new(|_, Pos { pos, .. }, textures: &Textures| {
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
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<OrNode>,
            draw_fn: Arc::new(|_, Pos { pos, .. }, textures: &Textures| {
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
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<NandNode>,
            draw_fn: Arc::new(|_, Pos { pos, .. }, textures: &Textures| {
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
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<NorNode>,
            draw_fn: Arc::new(|_, Pos { pos, .. }, textures: &Textures| {
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
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<XorNode>,
            draw_fn: Arc::new(|_, Pos { pos, .. }, textures: &Textures| {
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
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<XnorNode>,
            draw_fn: Arc::new(|_, Pos { pos, .. }, textures: &Textures| {
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
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<Wire>,
            draw_fn: Arc::new(|_, Pos { pos, .. }, _: &Textures| {
                draw_circle(pos.x, pos.y, 10.0, WHITE);
            }),
        })
        .with_thread_local(DrawNodeSys {
            node: PhantomData::<SwitchNode>,
            draw_fn: Arc::new(|node: &SwitchNode, Pos { pos, .. }, _: &Textures| {
                let color = if node.state { RED } else { WHITE };
                draw_rectangle(pos.x - 30.0, pos.y - 30.0, 60.0, 60.0, WHITE);
                draw_circle(pos.x, pos.y, 25.0, color);
                draw_circle_lines(pos.x, pos.y, 25.0, 2.5, BLACK);
            }),
        })
        .with_thread_local(DrawConnectionSys)
}
