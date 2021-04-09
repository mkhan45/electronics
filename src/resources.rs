use macroquad::texture::Texture2D;
use specs::Entity;

use crate::components::nodes::NodeTy;

#[derive(Default)]
pub struct TickProgress(pub f64);

#[derive(Default)]
pub struct Textures(pub std::collections::BTreeMap<String, Texture2D>);

#[derive(Copy, Clone)]
pub enum UIState {
    AddingNode(NodeTy),
    AddingWire {
        connection_entity: Entity,
        wire_entity: Entity,
        x_pos: Option<f32>,
        y_pos: Option<f32>,
    },
    Deleting,
    Nothing,
}

impl Default for UIState {
    fn default() -> Self {
        UIState::Nothing
    }
}

#[derive(Clone)]
pub enum UiSignal {
    AddNode(NodeTy),
    Delete,
}

#[derive(Default)]
pub struct UiSignals(pub Vec<UiSignal>);

#[derive(Clone, Copy, Default)]
pub struct Tick(pub usize);

impl Tick {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}

#[derive(Clone, Copy, Default)]
pub struct TickFrames(pub usize);

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum GridMode {
    Lines,
    Dots,
    CrossHatches,
    Off,
}

impl Default for GridMode {
    fn default() -> Self {
        GridMode::CrossHatches
    }
}

pub struct CurrentModeText(pub String);

impl Default for CurrentModeText {
    fn default() -> Self {
        CurrentModeText(
            "Right click a node to add a wire or click on the menu to choose a node".to_string(),
        )
    }
}
