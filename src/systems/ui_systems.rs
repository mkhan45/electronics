use crate::nodes::SwitchNode;
use crate::Connected;
use crate::Pos;
use specs::prelude::*;

use macroquad::prelude::*;

use crate::resources::{AddingNode, AddingWire, CurrentModeText};

pub struct CurrentModeSys;
impl<'a> System<'a> for CurrentModeSys {
    type SystemData = (
        Write<'a, CurrentModeText>,
        Read<'a, AddingNode>,
        Read<'a, AddingWire>,
    );

    fn run(&mut self, (mut current_mode, adding_node, adding_wire): Self::SystemData) {
        match adding_node.0 {
            Some(_) => {
                current_mode.0 = "Click to place node".to_string();
                return;
            }
            _ => {}
        };

        match adding_wire.0 {
            Some((_, _, None, Some(_))) => {
                current_mode.0 = "Right click to set wire position".to_string();
                return;
            }
            Some((_, _, Some(_), Some(_))) => {
                current_mode.0 =
                    "Right click a node to set wire output or left click to cancel".to_string();
                return;
            }
            _ => {}
        };

        *current_mode = CurrentModeText::default();
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
            .find(|(_, pos)| dbg!(pos.pos - mouse_pos).length() < 35.0);

        if let Some((s, _)) = target_switch {
            s.node.state = !s.node.state;
        }
    }
}
