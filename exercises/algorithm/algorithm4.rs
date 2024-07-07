/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

// use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone, // T needs to implement Clone
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO

        let new_node = Box::new(TreeNode::new(value.clone()));
        match &mut self.root {
            None => self.root = Some(new_node),
            Some(node) => node.insert(value),
        }
    }
    // fn insert(&mut self, value: T) {
    //     match self.root {
    //         None => self.root = Some(Box::new(TreeNode::new(value))),
    //         Some(ref mut root) => root.insert(value),
    //     }
    // }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match &self.root {
            None => false,
            Some(node) => node.search(value),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord + Clone, // T needs to implement Clone
{
    // fn new(value: T) -> Self {
    //     TreeNode {
    //         value,
    //         left: None,
    //         right: None,
    //     }
    // }
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO

        if value < self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        } else if value > self.value {
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }
    }
    fn search(&self, value: T) -> bool {
        if value < self.value {
            match &self.left {
                Some(left) => left.as_ref().search(value),
                None => false,
            }
        } else if value > self.value {
            match &self.right {
                Some(right) => right.as_ref().search(value),
                None => false,
            }
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
