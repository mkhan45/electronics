use super::Node;
use crate::systems::simulation_systems::ElectroSys;
use crate::systems::simulation_systems::WireSys;
use macroquad::prelude::Vec2;
use specs::{prelude::*, Component};

#[derive(Component, Default)]
pub struct Wire {
    pub input_state: bool,
    pub output_state: bool,
    pub changed_input: bool,
    pub start_point: Vec2,
    pub end_point: Vec2,
    pub points: Vec<Vec2>,
}

impl Node<1, 1> for Wire {
    fn calculate_state(&self, i: [bool; 1]) -> [bool; 1] {
        i
    }
}

#[derive(Clone, Copy)]
pub enum NodeTy {
    Wire,
    OnNode,
    OffNode,
    NotNode,
    AndNode,
    OrNode,
    NandNode,
    NorNode,
    XorNode,
    XnorNode,
    SwitchNode,
}

#[derive(Default)]
pub struct OnNode;
impl Node<0, 1> for OnNode {
    fn calculate_state(&self, _: [bool; 0]) -> [bool; 1] {
        [true]
    }
}

#[derive(Default)]
pub struct OffNode;
impl Node<0, 1> for OffNode {
    fn calculate_state(&self, _: [bool; 0]) -> [bool; 1] {
        [false]
    }
}

#[derive(Default)]
pub struct NotNode;
impl Node<1, 1> for NotNode {
    fn calculate_state(&self, input: [bool; 1]) -> [bool; 1] {
        [!input[0]]
    }

    fn input_offsets() -> [Vec2; 1] {
        [Vec2::new(-30.0, 0.0)]
    }

    fn output_offsets() -> [Vec2; 1] {
        [Vec2::new(22.5, 0.0)]
    }
}

#[derive(Default)]
pub struct AndNode;
impl Node<2, 1> for AndNode {
    fn calculate_state(&self, input: [bool; 2]) -> [bool; 1] {
        [input[0] && input[1]]
    }

    fn input_offsets() -> [Vec2; 2] {
        [Vec2::new(-35.0, -20.0), Vec2::new(-35.0, 20.0)]
    }

    fn output_offsets() -> [Vec2; 1] {
        [Vec2::new(35.0, 0.0)]
    }
}

#[derive(Default)]
pub struct OrNode;
impl Node<2, 1> for OrNode {
    fn calculate_state(&self, input: [bool; 2]) -> [bool; 1] {
        [input[0] || input[1]]
    }

    fn input_offsets() -> [Vec2; 2] {
        [Vec2::new(-35.0, -20.0), Vec2::new(-35.0, 20.0)]
    }

    fn output_offsets() -> [Vec2; 1] {
        [Vec2::new(35.0, 0.0)]
    }
}

#[derive(Default)]
pub struct NandNode;
impl Node<2, 1> for NandNode {
    fn calculate_state(&self, input: [bool; 2]) -> [bool; 1] {
        [!(input[0] && input[1])]
    }

    fn input_offsets() -> [Vec2; 2] {
        [Vec2::new(-35.0, -20.0), Vec2::new(-35.0, 20.0)]
    }

    fn output_offsets() -> [Vec2; 1] {
        [Vec2::new(35.0, 0.0)]
    }
}

#[derive(Default)]
pub struct NorNode;
impl Node<2, 1> for NorNode {
    fn calculate_state(&self, input: [bool; 2]) -> [bool; 1] {
        [!(input[0] || input[1])]
    }

    fn input_offsets() -> [Vec2; 2] {
        [Vec2::new(-35.0, -20.0), Vec2::new(-35.0, 20.0)]
    }

    fn output_offsets() -> [Vec2; 1] {
        [Vec2::new(35.0, 0.0)]
    }
}

#[derive(Default)]
pub struct XorNode;
impl Node<2, 1> for XorNode {
    fn calculate_state(&self, input: [bool; 2]) -> [bool; 1] {
        [input[0] ^ input[1]]
    }

    fn input_offsets() -> [Vec2; 2] {
        [Vec2::new(-35.0, -20.0), Vec2::new(-35.0, 20.0)]
    }

    fn output_offsets() -> [Vec2; 1] {
        [Vec2::new(35.0, 0.0)]
    }
}

#[derive(Default)]
pub struct XnorNode;
impl Node<2, 1> for XnorNode {
    fn calculate_state(&self, input: [bool; 2]) -> [bool; 1] {
        [!(input[0] ^ input[1])]
    }

    fn input_offsets() -> [Vec2; 2] {
        [Vec2::new(-35.0, -20.0), Vec2::new(-35.0, 20.0)]
    }

    fn output_offsets() -> [Vec2; 1] {
        [Vec2::new(35.0, 0.0)]
    }
}

#[derive(Default)]
pub struct SwitchNode {
    pub state: bool,
}

impl Node<0, 1> for SwitchNode {
    fn calculate_state(&self, _input: [bool; 0]) -> [bool; 1] {
        [self.state]
    }

    fn output_offsets() -> [Vec2; 1] {
        [Vec2::new(35.0, 0.0)]
    }
}

#[macro_export]
macro_rules! all_nodes {
    ($macro:ident) => {
        $macro!(
            [OnNode, 0, 1],
            [OffNode, 0, 1],
            [Wire, 1, 1],
            [NotNode, 1, 1],
            [AndNode, 2, 1],
            [OrNode, 2, 1],
            [NandNode, 2, 1],
            [NorNode, 2, 1],
            [XorNode, 2, 1],
            [XnorNode, 2, 1],
            [SwitchNode, 0, 1],
        )
    };
}

pub fn add_node_systems<'a, 'b>(builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    macro_rules! add_systems {
        ( $([$node:ident, $i:expr, $o:expr]),* $(,)? ) => {
            builder
                .with(WireSys, "wire_sys", &[])
                $(
                    .with(ElectroSys::<$node, $i, $o>::default(), stringify!($node), &["wire_sys"])
                )*
        };
    }

    all_nodes!(add_systems)
}
