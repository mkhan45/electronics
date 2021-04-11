# Connection rework

Right now, wire inputs are pushed to by a node, and then the outputs pull from the input, and then the next node pulls from the output.

With Connections, the node can push to the wire input through the connection, and then the node pulls from the wire output through a connection as well.

This makes it easy to iterate through all the connections at once.

# New wires

Add Hitbox component

```rs
enum Hitbox {
    Circle(f32), // for nodes/connections
    Rect(Rect), // for wires
    Compound(Vec<Hitbox>), // for wires
}
```

Make all click stuff go thru hitboxes

Remove Pos component from wires, instead

```rs
pub struct Wire {
    ...
    bend_points: Vec<Vec2>,
}
```

Rework drawing wires
