use crate::nodes::NodeTy;
use crate::{components::ConnectionTy, Pos};
use macroquad::prelude::Vec2;
use rhai::Map;
use std::{collections::BTreeMap, convert::TryInto, marker::PhantomData};

use rhai::{self, Array, Dynamic, Identifier};
use specs::prelude::*;

use crate::{
    components::{nodes::Wire, Connected, Connection, Node, Orientation},
    resources::{RhaiEngine, RhaiScope},
};

// Ok so the logical graph structure is potentially cyclic and pretty wonky actually so it's
// impossible to describe it just using straightforward JSON or similar
//
// This basically just seeks to make the Rhai code mirror the Rust circuit creation; it's not great
// for humans to write but it's fine for auto serialization and deserialization

pub fn create_circuit(world: &mut World) {
    // 1. Create wires - done
    // 2. Get RhaiNodes which contain a type, [[input wires]], [[output wires]] - done
    // 3. Create Nodes and Connections
    // 4. Profit

    let wires: Map = {
        let engine = world.fetch::<RhaiEngine>();
        let mut scope = world.fetch_mut::<RhaiScope>();
        engine.0.eval_with_scope(&mut scope.0, "WIRES").unwrap()
    };

    let wires = wires
        .iter()
        .map(|(name, wire)| {
            let wire = wire.clone_cast::<Map>();
            let bends = wire
                .get("bends")
                .unwrap()
                .clone_cast::<Array>()
                .iter()
                .map(|point| {
                    let point = point.clone_cast::<Map>();
                    let x = point.get("x").unwrap().clone();
                    let y = point.get("y").unwrap().clone();
                    Vec2::new(x.cast::<f32>(), y.cast::<f32>())
                })
                .collect();

            (
                name.to_string(),
                Wire {
                    points: bends,
                    ..Wire::default()
                },
            )
        })
        .map(|(name, wire)| {
            (
                name.clone(),
                world.create_entity().with(wire.clone()).build(),
            )
        })
        .collect::<BTreeMap<String, Entity>>();

    struct RhaiNode {
        pub ty: NodeTy,
        pub input_wires: Vec<Vec<String>>,
        pub output_wires: Vec<Vec<String>>,
        pub pos: Vec2,
    }

    let nodes: Array = {
        let engine = world.fetch::<RhaiEngine>();
        let mut scope = world.fetch_mut::<RhaiScope>();
        engine.0.eval_with_scope(&mut scope.0, "NODES").unwrap()
    };

    let nodes = nodes.iter().map(|node: &Dynamic| {
        let node = node.clone_cast::<Map>();

        let ty = node.get("type").unwrap();
        let ty = ty.clone_cast::<String>();

        let ty = {
            use crate::components::nodes::NodeTy::*;

            match ty.as_str() {
                "On" => OnNode,
                "Off" => OffNode,
                "Not" => NotNode,
                "And" => AndNode,
                "Or" => OrNode,
                "Nand" => NandNode,
                "Nor" => NorNode,
                "Xor" => XorNode,
                "Xnor" => XnorNode,
                "Switch" => SwitchNode,
                _ => panic!("Invalid Node"),
            }
        };

        let process_array = |name: &str| {
            let arr = node.get(name).unwrap();
            let arr = arr.clone_cast::<Array>();
            arr.iter()
                .map(|connection| {
                    let connection = connection.clone_cast::<Array>();
                    connection
                        .iter()
                        .map(|input| input.clone_cast::<String>())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>()
        };

        let input_wires = process_array("inputs");
        let output_wires = process_array("inputs");

        let pos = node.get("pos").unwrap().clone_cast::<Map>();
        let x = pos.get("x").unwrap().clone_cast::<f32>();
        let y = pos.get("y").unwrap().clone_cast::<f32>();
        let pos = Vec2::new(x, y);

        RhaiNode {
            ty,
            input_wires,
            output_wires,
            pos,
        }
    });
}

// pub struct CreateScriptedCircuitSys<N, const I: usize, const O: usize>
// where
//     N: Node<I, O> + 'static,
// {
//     node: PhantomData<N>,
//     node_name: String,
// }

// impl<'a, N, const I: usize, const O: usize> System<'a> for CreateScriptedCircuitSys<N, I, O>
// where
//     N: Node<I, O> + 'static,
// {
//     type SystemData = (
//         Read<'a, RhaiEngine>,
//         Write<'a, RhaiScope<'static>>,
//         WriteStorage<'a, Connected<N, I, O>>,
//         WriteStorage<'a, Pos>,
//         WriteStorage<'a, Wire>,
//         WriteStorage<'a, Connection>,
//         Entities<'a>,
//     );

//     fn run(
//         &mut self,
//         (engine, mut scope, mut nodes, mut positions, mut wires, mut connections, entities): Self::SystemData,
//     ) {
//         let circuit: Array = engine
//             .0
//             .eval_with_scope(&mut scope.0, "CIRCUIT")
//             .unwrap_or(Array::new())
//             .to_owned();

//         circuit.iter().cloned().for_each(|node: Dynamic| {
//             let map = node.cast::<Map>();
//             let ty = map.get("type").unwrap();
//             let type_str = ty.clone().take_immutable_string().unwrap();
//             if type_str == self.node_name {
//                 let pos = map.get("pos").unwrap();
//                 let pos_arr = pos.clone_cast::<Array>();
//                 let x = pos_arr[0].clone_cast::<f64>();
//                 let y = pos_arr[1].clone_cast::<f64>();

//                 let pos = Pos::from_vec((x as f32, y as f32).into());
//                 let input_offsets = N::input_offsets();
//                 let output_offsets = N::output_offsets();

//                 let inputs = (0..I)
//                     .map(|index| {
//                         entities
//                             .build_entity()
//                             .with(
//                                 Connection {
//                                     wires: Vec::new(),
//                                     ty: ConnectionTy::Input,
//                                     index,
//                                 },
//                                 &mut connections,
//                             )
//                             .with(
//                                 Pos::from_vec_unrounded(pos.pos + input_offsets[index]),
//                                 &mut positions,
//                             )
//                             .build()
//                     })
//                     .collect::<Vec<_>>()
//                     .try_into()
//                     .unwrap();

//                 let outputs = (0..O)
//                     .map(|index| {
//                         entities
//                             .build_entity()
//                             .with(
//                                 Connection {
//                                     wires: Vec::new(),
//                                     ty: ConnectionTy::Output,
//                                     index,
//                                 },
//                                 &mut connections,
//                             )
//                             .with(
//                                 Pos::from_vec_unrounded(pos.pos + output_offsets[index]),
//                                 &mut positions,
//                             )
//                             .build()
//                     })
//                     .collect::<Vec<_>>()
//                     .try_into()
//                     .unwrap();

//                 entities
//                     .build_entity()
//                     .with(
//                         Connected {
//                             node: N::default(),
//                             inputs,
//                             outputs,
//                         },
//                         &mut nodes,
//                     )
//                     .with(pos, &mut positions)
//                     .build();
//             }
//         });
//     }
// }

// pub fn create_circuit(circuit: Array, world: &World) {
//     fn create_node(node: &Dynamic, world: &World) -> Entity {
//         let map = node.cast::<Map>();

//         let outputs = {
//             let outputs = map.get("outputs").unwrap();
//             let output_arr = outputs.clone_cast::<Array>();
//             output_arr
//                 .iter()
//                 .map(|node| create_node(node, world))
//                 .collect::<Vec<_>>();
//         };
//         let pos = {
//             let pos = map.get("pos").unwrap();
//             let pos_arr = pos.clone_cast::<Array>();
//             Pos::from_vec(Vec2::new(
//                 pos_arr[0].clone_cast::<f32>(),
//                 pos_arr[1].clone_cast::<f32>(),
//             ))
//         };

//         // We have a list of output node entities
//         todo!();
//     }

//     circuit.iter().cloned().for_each(|node: Dynamic| {
//         create_node(&node, world);
//     });
// }

// pub fn run_circuit_create_sys(script: String, world: &World) {
//     use crate::all_nodes;
//     use crate::nodes::*;

//     {
//         let engine = &world.fetch::<RhaiEngine>().0;
//         let scope = &mut world.fetch_mut::<RhaiScope>().0;

//         engine.eval_with_scope::<()>(scope, &script).unwrap();
//     }

//     macro_rules! run_sys {
//         ( $([$node:ident, $i:expr, $o:expr]),* $(,)? ) => {
//             $(
//                 CreateScriptedCircuitSys { node: PhantomData::<$node>, node_name: stringify!($node).to_string() }.run_now(world);
//             )*
//         };
//     }

//     all_nodes!(run_sys);
// }
