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
                    let leftH = dfs(root.borrow().left.clone());
                    let rightH = dfs(root.borrow().right.clone());
                    if leftH == -1 || rightH == -1 || (leftH - rightH).abs() > 1 {
                        return -1
                    }
                    leftH.max(rightH) + 1
                }

                None => 0
            }
        }
        dfs(root) != -1
    }
}