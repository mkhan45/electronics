use crate::nodes::SwitchNode;
use crate::resources::UIState;
use crate::Connected;
use crate::Pos;
use specs::prelude::*;

use macroquad::prelude::*;

use crate::resources::CurrentModeText;

pub struct CurrentModeSys;
impl<'a> System<'a> for CurrentModeSys {
    type SystemData = (Write<'a, CurrentModeText>, Read<'a, UIState>);

    fn run(&mut self, (mut current_mode, ui_state): Self::SystemData) {
        match *ui_state {
            UIState::AddingNode(_) => {
                current_mode.0 = "Click to place node".to_string();
            }
            UIState::AddingWire {
                x_pos: None,
                y_pos: Some(_),
                ..
            } => {
                current_mode.0 = "Right click to set wire position".to_string();
            }
            UIState::AddingWire {
                x_pos: Some(_),
                y_pos: Some(_),
                ..
            } => {
                current_mode.0 =
                    "Right click a node to set wire output or left click to cancel".to_string();
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
        ReadStorage<'a, Pos>,
    );

    fn run(&mut self, (mut switches, positions): Self::SystemData) {
        let mouse_pos = {
            let (mx, my) = mouse_position();
            Vec2::new(mx, my)
        };

        let target_switch = (&mut switches, &positions)
            .join()
            .find(|(_, pos)| (pos.pos - mouse_pos).length() < 35.0);

        if let Some((s, _)) = target_switch {
            s.node.state = !s.node.state;
        }
    }
}
