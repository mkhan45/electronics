use macroquad::{prelude::Vec2, texture::Texture2D};
use specs::Entity;

use crate::components::nodes::NodeTy;

#[derive(Default)]
pub struct TickProgress(pub f64);

#[derive(Default)]
pub struct Textures(pub std::collections::BTreeMap<String, Texture2D>);

#[derive(Default)]
pub struct AddingNode(pub Option<NodeTy>);

#[derive(Default)]
pub struct AddingWire(pub Option<(Entity, Option<Vec2>)>);

#[derive(Clone)]
pub enum UiSignal {
    AddNode(NodeTy),
}

#[derive(Default)]
pub struct UiSignals(pub Vec<UiSignal>);

#[derive(Clone, Copy)]
pub struct Tick(pub usize);

impl Tick {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}
