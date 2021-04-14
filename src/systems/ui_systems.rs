use crate::resources::UIState;
use crate::Connected;
use crate::Pos;
use crate::{nodes::SwitchNode, resources::MousePos};
use specs::prelude::*;

use crate::resources::CurrentModeText;

pub struct CurrentModeSys;
impl<'a> System<'a> for CurrentModeSys {
    type SystemData = (Write<'a, CurrentModeText>, Read<'a, UIState>);

    fn run(&mut self, (mut current_mode, ui_state): Self::SystemData) {
        match *ui_state {
            UIState::AddingNode(_) => {
                current_mode.0 = "Click to place node".to_string();
            }
            UIState::AddingWire { .. } => {
                current_mode.0 = "Right click to add a bend or complete the connection".to_string();
            }
            UIState::Deleting => {
                current_mode.0 = "Click a node or wire focus to delete it".to_string();
            }
            _ => {
                *current_mode = CurrentModeText::default();
            }
        };
    }
}

pub struct SwitchClickSys;
impl<'a> System<'a> for SwitchClickSys {
    type SystemData = (
        WriteStorage<'a, Connected<SwitchNode, 0, 1>>,
        Read<'a, MousePos>,
        ReadStorage<'a, Pos>,
    );

    fn run(&mut self, (mut switches, mouse_pos, positions): Self::SystemData) {
        let mouse_pos = mouse_pos.0;

        let target_switch = (&mut switches, &positions)
            .join()
            .find(|(_, pos)| (pos.pos - mouse_pos).length() < 35.0);

        if let Some((s, _)) = target_switch {
            s.node.state = !s.node.state;
        }
    }
}
