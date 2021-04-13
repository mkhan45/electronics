# Connection rework

Right now, wire inputs are pushed to by a node, and then the outputs pull from the input, and then the next node pulls from the output.

With Connections, the node can push to the wire input through the connection, and then the node pulls from the wire output through a connection as well.

This makes it easy to iterate through all the connections at once.

# New wires

Add Hitbox component

```rs
#[derive(Clone, Component)]
pub enum Hitbox {
    Circle { center: Vec2, radius: f32 },
    Rectangle { bounds: Rect },
    Compound { inner: Vec<Hitbox> },
}

impl Hitbox {
    pub fn contains(&self, point: Vec2) -> bool {
        use Hitbox::*;

        match self {
            Circle { center, radius } => (point - *center).length() <= *radius,
            Rectangle { bounds } => bounds.contains(point),
            Compound { inner } => inner.iter().any(|hb| hb.contains(point)),
        }
    }
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
