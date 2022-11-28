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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn node_sum(node: &Option<Rc<RefCell<TreeNode>>>, sum_tilts: &mut i32) -> i32 {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    let l = node_sum(&node.left, sum_tilts);
                    let r = node_sum(&node.right, sum_tilts);
                    *sum_tilts += (l-r).abs();
                    l + r + node.val
                }

                None => 0
            }
        }
        let mut sum = 0;
        node_sum(&root, &mut sum);
        return sum
        
    }
}