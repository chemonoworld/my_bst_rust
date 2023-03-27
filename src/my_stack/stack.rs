use crate::my_node::node::Node;


pub struct Stack {
    pub stack: Vec<Node>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, node: Node) {
        self.stack.push(node);
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&Node> {
        self.stack.last()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        assert_eq!(stack.is_empty(), true);
        stack.push(Node::new(1));
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.peek().unwrap().get_value(), 1);
        stack.push(Node::new(2));
        assert_eq!(stack.peek().unwrap().get_value(), 2);
        stack.push(Node::new(3));
        assert_eq!(stack.peek().unwrap().get_value(), 3);
        assert_eq!(stack.pop().unwrap().get_value(), 3);
        assert_eq!(stack.pop().unwrap().get_value(), 2);
        assert_eq!(stack.pop().unwrap().get_value(), 1);
        assert_eq!(stack.is_empty(), true);
    }
}