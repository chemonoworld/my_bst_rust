use crate::my_node::node::Node;

pub struct BinarySearchTree {
    pub root: Option<Box<Node>>,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
        }
    }

    pub fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;
        while let Some(node) = current {
            if value < node.get_value() {
                current = &mut node.left;
            } else {
                current = &mut node.right;
            }
        }
        *current = Some(Box::new(Node::new(value)));
    }

    pub fn search(&self, value: i32) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if value == node.get_value() {
                return true;
            } else if value < node.get_value() {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(2);
        bst.insert(7);
        bst.insert(12);
        bst.insert(17);
        assert_eq!(bst.search(10), true);
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(15), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(12), true);
        assert_eq!(bst.search(17), true);
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(3), false);
        assert_eq!(bst.search(6), false);
        assert_eq!(bst.search(8), false);
        assert_eq!(bst.search(11), false);
        assert_eq!(bst.search(13), false);
        assert_eq!(bst.search(16), false);
        assert_eq!(bst.search(18), false);
    }
}