use super::Node;

pub struct OnNode;
impl Node<0, 1> for OnNode {
    fn calculate_state(_: [bool; 0]) -> [bool; 1] {
        [true]
    }
}

pub struct OffNode;
impl Node<0, 1> for OffNode {
    fn calculate_state(_: [bool; 0]) -> [bool; 1] {
        [false]
    }
}

pub struct Wire;
impl Node<1, 1> for Wire {
    fn calculate_state(input: [bool; 1]) -> [bool; 1] {
        [input[0]]
    }
}

pub struct NotNode;
impl Node<1, 1> for NotNode {
    fn calculate_state(input: [bool; 1]) -> [bool; 1] {
        [!input[0]]
    }
}
