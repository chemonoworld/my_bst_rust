pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn get_left(&self) -> Option<&Box<Node>> {
        self.left.as_ref()
    }

    pub fn set_left(&mut self, left: Option<Box<Node>>) {
        self.left = left;
    }

    pub fn get_right(&self) -> Option<&Box<Node>> {
        self.right.as_ref()
    }

    pub fn set_right(&mut self, right: Option<Box<Node>>) {
        self.right = right;
    }
}
