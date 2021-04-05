use specs::prelude::*;

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
