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
use crate::{resources::CameraRes, Wire};
use crate::{resources::GridMode, Connected};
use crate::{resources::MousePos, Pos};
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
        Read<'a, Textures>,
    );

    fn run(&mut self, (positions, nodes, textures): Self::SystemData) {
        (&positions, &nodes).join().for_each(|(self_pos, node)| {
            (self.draw_fn)(&node.node, *self_pos, &textures);
        });
    }
}

// to help drawing wires
struct Points<'a> {
    pub start_point: &'a Vec2,
    pub end_point: &'a Vec2,
    pub points: &'a Vec<Vec2>,
}

impl<'a> std::ops::Index<usize> for Points<'a> {
    type Output = Vec2;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => self.start_point,
            i if i == self.points.len() + 1 => self.end_point,
            i if i < self.points.len() + 1 => &self.points[i - 1],
            _ => panic!(
                "Invalid Index for points with len {}: {}",
                self.points.len() + 1,
                i
            ),
        }
    }
}

// ideally should make PointsIterator or similar but this works fine
impl<'a> Points<'a> {
    pub fn for_each(&self, mut f: impl FnMut(&Vec2, &Vec2)) {
        (0..self.points.len() + 1).for_each(|i| {
            let a = self[i];
            let b = self[i + 1];
            f(&a, &b);
        });
    }
}

pub struct DrawWireSys;
impl<'a> System<'a> for DrawWireSys {
    type SystemData = (ReadStorage<'a, Wire>, Read<'a, TickProgress>);

    fn run(&mut self, (wires, tick_progress): Self::SystemData) {
        wires.join().for_each(|wire| {
            let points = Points {
                start_point: &wire.start_point,
                end_point: &wire.end_point,
                points: &wire.points,
            };

            let mut total_len = 0.0;
            points.for_each(|sp, ep| total_len += (ep.x - sp.x).abs() + (ep.y - sp.y).abs());

            let mut new_col_len_remaining = tick_progress.0 as f32 * total_len;

            if wire.output_state == wire.input_state {
                new_col_len_remaining = total_len;
            }

            let (new_col, old_col) = if wire.input_state {
                (RED, WHITE)
            } else {
                (WHITE, RED)
            };

            points.for_each(|sp, ep| {
                // vertical
                if new_col_len_remaining > (ep.y - sp.y).abs() {
                    draw_line(sp.x, sp.y, sp.x, ep.y, 5.0, new_col);
                    new_col_len_remaining -= (sp.y - ep.y).abs();
                } else if new_col_len_remaining <= 0.0 {
                    draw_line(sp.x, sp.y, sp.x, ep.y, 5.0, old_col);
                } else {
                    let diff = (ep.y - sp.y).signum();
                    let midpoint = new_col_len_remaining * diff + sp.y;

                    draw_line(sp.x, sp.y, sp.x, midpoint, 5.0, new_col);
                    draw_line(sp.x, midpoint, sp.x, ep.y, 5.0, old_col);
                    new_col_len_remaining = 0.0
                }

                // horizontal
                if new_col_len_remaining > (ep.x - sp.x).abs() {
                    draw_line(sp.x, ep.y, ep.x, ep.y, 5.0, new_col);
                    draw_circle(sp.x, ep.y, 5.0, new_col);
                    draw_circle(ep.x, ep.y, 5.0, new_col);
                    new_col_len_remaining -= (sp.x - ep.x).abs();
                } else if new_col_len_remaining <= 0.0 {
                    draw_line(sp.x, ep.y, ep.x, ep.y, 5.0, old_col);
                    draw_circle(sp.x, ep.y, 5.0, old_col);
                    draw_circle(ep.x, ep.y, 5.0, old_col);
                } else {
                    let diff = (ep.x - sp.x).signum();
                    let midpoint = new_col_len_remaining * diff + sp.x;

                    draw_line(sp.x, ep.y, midpoint, ep.y, 5.0, new_col);
                    draw_line(midpoint, ep.y, ep.x, ep.y, 5.0, old_col);
                    draw_circle(sp.x, ep.y, 5.0, new_col);
                    draw_circle(ep.x, ep.y, 5.0, old_col);
                    new_col_len_remaining = 0.0
                }
            });
        });
    }
}

pub struct TempWireDrawSys;
impl<'a> System<'a> for TempWireDrawSys {
    type SystemData = (Read<'a, UIState>, ReadStorage<'a, Pos>, Read<'a, MousePos>);

    fn run(&mut self, (ui_state, position_storage, mouse_pos): Self::SystemData) {
        use crate::components::round_to_snap as snap;

        let color = LIGHTGRAY;
        let mut mouse_pos = mouse_pos.0;
        mouse_pos.x = snap(mouse_pos.x);
        mouse_pos.y = snap(mouse_pos.y);

        if let UIState::AddingWire {
            points: wire_points,
            connection_entity,
        } = &*ui_state
        {
            let start_point = &position_storage.get(*connection_entity).unwrap().pos;

            let points = Points {
                start_point,
                points: &wire_points,
                end_point: &mouse_pos,
            };

            points.for_each(|sp, ep| {
                // horizontal
                draw_line(sp.x, ep.y, ep.x, ep.y, 5.0, color);

                // vertical
                draw_line(sp.x, sp.y, sp.x, ep.y, 5.0, color);

                draw_circle(sp.x, ep.y, 5.0, color);
            });
        }
    }
}

pub struct DrawConnectionSys;
impl<'a> System<'a> for DrawConnectionSys {
    type SystemData = (
        ReadStorage<'a, Connection>,
        ReadStorage<'a, Pos>,
        Read<'a, MousePos>,
    );

    fn run(&mut self, (connections, positions, mouse_pos): Self::SystemData) {
        let mouse_pos = mouse_pos.0;

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
    type SystemData = (Read<'a, GridMode>, Read<'a, CameraRes>);

    // TODO: for some reason this only draws in the first quadrant, should be fixed later
    fn run(&mut self, (grid_mode, camera_res): Self::SystemData) {
        use crate::components::SNAP;
        let s = 4;
        let camera = camera_res.0;

        let (top, left) = {
            let top_left = camera.screen_to_world((0.0, 0.0).into());
            (top_left.y, top_left.x)
        };
        let top = top - top.ceil() % SNAP + SNAP;
        let left = left - left.ceil() % SNAP - SNAP;
        let sx = s - ((left.floor() / SNAP).abs() as usize % s);

        let (bottom, right) = {
            let bottom_right = camera.screen_to_world((screen_width(), screen_height()).into());
            (bottom_right.y, bottom_right.x)
        };
        let bottom = bottom - bottom.ceil() % SNAP - SNAP;
        let right = right - right.ceil() % SNAP + SNAP;

        // idk why they're switched but they are
        let (bottom, top) = (top, bottom);
        let sy = s - ((top.floor() / SNAP).abs() as usize % s);

        let x_positions = (0..((right - left) / SNAP).ceil() as usize + s + 1)
            .map(|i| (i, i % s == 0))
            .map(|(i, is_big)| (left + i as f32 * SNAP - sx as f32 * SNAP - SNAP, is_big));
        let y_positions = (0..((bottom - top) / SNAP).ceil() as usize + s + 1)
            .map(|i| (i, i % s == 0))
            .map(|(i, is_big)| (top + i as f32 * SNAP - sy as f32 * SNAP, is_big));

        match *grid_mode {
            GridMode::Lines => {
                // lines
                let base_width = 0.5;
                let wider_width = 1.5;

                let thickness = |is_big| {
                    if is_big {
                        wider_width
                    } else {
                        base_width
                    }
                };

                x_positions.for_each(|(x, is_big)| {
                    draw_line(x, top, x, bottom, thickness(is_big), DARKGRAY)
                });
                y_positions.for_each(|(y, is_big)| {
                    draw_line(left, y, right, y, thickness(is_big), DARKGRAY)
                });
            }
            GridMode::Dots => {
                let base_rad = 1.5;
                let wider_rad = 3.0;

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
        .with_thread_local(DrawGridSys)
        .with_thread_local(DrawWireSys)
        .with_thread_local(TempWireDrawSys)
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
