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
   leftH := dfs(root.Left)
   rightH := dfs(root.Right)

   if leftH == -1 || rightH == -1 || abs(leftH - rightH) > 1 {
       return -1
   }

   return max(leftH, rightH) + 1
}

func max(a, b int) int {
   if a > b {
       return a
   }
   return b
}

func abs(n int) int {
   if n > 0 {
       return n
   }
   return -n
}
