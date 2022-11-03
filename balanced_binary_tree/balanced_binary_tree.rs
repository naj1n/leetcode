// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(root) => {
                    let leftHeight = dfs(root.borrow().left.clone());
                    let rightHeight = dfs(root.borrow().right.clone());
                    if (leftHeight - rightHeight).abs() > 1 || leftHeight == -1 || rightHeight == -1 {
                        return -1
                    }
                    leftHeight.max(rightHeight) + 1
                }
                None => 0
            }
        }
    dfs(root) != -1
    }
}