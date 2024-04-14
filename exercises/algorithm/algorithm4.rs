/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
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

impl<T: std::cmp::PartialOrd> BinarySearchTree<T>
where
    T: Ord + Debug,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        // 左边的 mut 说明可以更换被引用的对象
        // 右边的 &mut 说明可以更改被引用的对象的内容
        // 整一个 Option 理解为 c++ 的指针
        // 问题：self 是 &mut，内部的 field 不会自动变为引用吗？
        let mut root = &mut self.root;
        while root.is_some() {
            if root.as_ref().unwrap().value == value {
                return;
            } else if value < root.as_ref().unwrap().value {
                // 先获得内部 box 的可变引用，进而获得 .left 和 .right 的可变引用
                // 每一层解构都使用引用
                root = &mut root.as_mut().unwrap().left;
            } else {
                root = &mut root.as_mut().unwrap().right;
            }
        }

        *root = Some(Box::new(TreeNode::new(value)))
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut root = &self.root;
        while root.is_some() && root.as_ref().unwrap().value != value {
            if value < root.as_ref().unwrap().value {
                root = &root.as_ref().unwrap().left;
            } else {
                root = &root.as_ref().unwrap().right;
            }
        }

        root.is_some() && root.as_ref().unwrap().value == value
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


