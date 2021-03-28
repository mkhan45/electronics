use super::Node;
use crate::{systems::ElectroSys, WireSys};
use specs::prelude::*;

pub struct Wire;
impl Node<1, 1> for Wire {
    fn calculate_state(input: [bool; 1]) -> [bool; 1] {
        [input[0]]
    }
}

pub struct OnNode;
impl Node<0, 1> for OnNode {
    fn calculate_state(_: [bool; 0]) -> [bool; 1] {
        [true]
    }
}

pub struct OffNode;
impl Node<0, 1> for OffNode {
    fn calculate_state(_: [bool; 0]) -> [bool; 1] {
        [false]
    }
}

pub struct NotNode;
impl Node<1, 1> for NotNode {
    fn calculate_state(input: [bool; 1]) -> [bool; 1] {
        [!input[0]]
    }
}

pub struct AndNode;
impl Node<2, 1> for AndNode {
    fn calculate_state(input: [bool; 2]) -> [bool; 1] {
        [input[0] && input[1]]
    }
}

pub struct OrNode;
impl Node<2, 1> for OrNode {
    fn calculate_state(input: [bool; 2]) -> [bool; 1] {
        [input[0] || input[1]]
    }
}

pub struct NandNode;
impl Node<2, 1> for NandNode {
    fn calculate_state(input: [bool; 2]) -> [bool; 1] {
        [!(input[0] && input[1])]
    }
}

pub struct NorNode;
impl Node<2, 1> for NorNode {
    fn calculate_state(input: [bool; 2]) -> [bool; 1] {
        [!(input[0] || input[1])]
    }
}

pub struct XorNode;
impl Node<2, 1> for XorNode {
    fn calculate_state(input: [bool; 2]) -> [bool; 1] {
        [input[0] ^ input[1]]
    }
}

pub struct XnorNode;
impl Node<2, 1> for XnorNode {
    fn calculate_state(input: [bool; 2]) -> [bool; 1] {
        [!(input[0] ^ input[1])]
    }
}

pub fn add_node_systems<'a, 'b>(builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    builder
        .with(WireSys, "wire_sys", &[])
        .with(
            ElectroSys::<OnNode, 0, 1>::default(),
            "on_node_sys",
            &["wire_sys"],
        )
        .with(
            ElectroSys::<OffNode, 0, 1>::default(),
            "off_node_sys",
            &["wire_sys"],
        )
        .with(
            ElectroSys::<NotNode, 1, 1>::default(),
            "not_node_sys",
            &["wire_sys"],
        )
        .with(
            ElectroSys::<AndNode, 2, 1>::default(),
            "and_node_sys",
            &["wire_sys"],
        )
        .with(
            ElectroSys::<OrNode, 2, 1>::default(),
            "or_node_sys",
            &["wire_sys"],
        )
        .with(
            ElectroSys::<NandNode, 2, 1>::default(),
            "nand_node_sys",
            &["wire_sys"],
        )
        .with(
            ElectroSys::<NorNode, 2, 1>::default(),
            "nor_node_sys",
            &["wire_sys"],
        )
        .with(
            ElectroSys::<XorNode, 2, 1>::default(),
            "xor_node_sys",
            &["wire_sys"],
        )
        .with(
            ElectroSys::<XnorNode, 2, 1>::default(),
            "xnor_node_sys",
            &["wire_sys"],
        )
}
