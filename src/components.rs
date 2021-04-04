use core::marker::PhantomData;
use macroquad::prelude::Vec2;
use specs::{prelude::*, Component};

pub mod nodes;

pub trait Node<const I: usize, const O: usize> {
    fn calculate_state(inputs: [bool; I]) -> [bool; O]
    where
        Self: Sized;
}

#[derive(Component)]
pub struct Connected<N, const I: usize, const O: usize>
where
    N: Node<I, O> + 'static,
{
    pub node: PhantomData<N>,
    pub inputs: [Option<Entity>; I],
    pub outputs: [Option<Entity>; O],
}

impl<N, const I: usize, const O: usize> Default for Connected<N, I, O>
where
    N: Node<I, O> + 'static,
{
    fn default() -> Self {
        Connected {
            node: PhantomData::<N>,
            inputs: [None; I],
            outputs: [None; O],
        }
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
