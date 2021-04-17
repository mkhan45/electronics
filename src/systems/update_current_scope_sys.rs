use crate::components::{CurrentScope, InnerNode};
use crate::resources::CreatingCompoundNode;
use specs::prelude::*;

pub struct UpdateCurrentScopeSys;

impl<'a> System<'a> for UpdateCurrentScopeSys {
    type SystemData = (
        Read<'a, CreatingCompoundNode>,
        ReadStorage<'a, InnerNode>,
        WriteStorage<'a, CurrentScope>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (creating_compound, inner_nodes, mut current_scopes, entities): Self::SystemData,
    ) {
        current_scopes.clear();

        if let Some(parent_node) = &creating_compound.0 {
            (&inner_nodes, &entities)
                .join()
                .filter(|(inner_node_data, _)| inner_node_data.parent == parent_node.entity)
                .for_each(|(_, entity)| {
                    current_scopes.insert(entity, CurrentScope).unwrap();
                });
        } else {
            (&entities, !&inner_nodes).join().for_each(|(entity, _)| {
                current_scopes.insert(entity, CurrentScope).unwrap();
            });
        }
    }
}
