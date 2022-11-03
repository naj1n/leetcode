/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
 func isBalanced(root *TreeNode) bool {
	return dfs(root) != -1
}

func dfs(root *TreeNode) int {
   if root == nil {
	   return 0
   }
   leftHeight := dfs(root.Left)
   rightHeight := dfs(root.Right)
   if abs(leftHeight - rightHeight) > 1 || leftHeight == -1 || rightHeight == -1 {
	   return -1
   }
   return max(leftHeight, rightHeight) + 1
   
}

func max(a,b int) int {
   if a > b {
	   return a
   }
   return b
}

func abs(n int) int {
   if n >= 0 {
	   return n
   }
   return -n
}