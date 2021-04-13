use macroquad::prelude::Vec2;
use specs::{prelude::*, Component};

pub mod nodes;

pub trait Node<const I: usize, const O: usize>: Default {
    fn calculate_state(&self, inputs: [bool; I]) -> [bool; O];
    fn input_offsets() -> [Vec2; I] {
        [Vec2::new(0.0, 0.0); I]
    }
    fn output_offsets() -> [Vec2; O] {
        [Vec2::new(0.0, 0.0); O]
    }
}

#[derive(Component)]
pub struct Connected<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    pub node: N,
    pub inputs: [Entity; I],
    pub outputs: [Entity; O],
}

impl<N, const I: usize, const O: usize> Connected<N, I, O>
where
    N: Node<I, O> + 'static,
{
    pub fn calculate_state(&self, inputs: [bool; I]) -> [bool; O] {
        self.node.calculate_state(inputs)
    }
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Clone, Copy)]
pub struct Pos {
    pub orientation: Orientation,
    pub pos: Vec2,
}

pub const SNAP: f32 = 75.0;

pub fn round_to_snap(x: f32) -> f32 {
    (x / SNAP).round() * SNAP
}

impl Pos {
    pub fn from_vec(p: Vec2) -> Self {
        let pos = Vec2::new((p.x / SNAP).round() * SNAP, (p.y / SNAP).round() * SNAP);
        Pos {
            orientation: Orientation::Right,
            pos,
        }
    }

    pub fn from_vec_unrounded(pos: Vec2) -> Self {
        Pos {
            orientation: Orientation::Right,
            pos,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ConnectionTy {
    Input,
    Output,
}

#[derive(Clone, Component)]
pub struct Connection {
    pub wires: Vec<Entity>,
    pub ty: ConnectionTy,
    pub index: usize,
}
