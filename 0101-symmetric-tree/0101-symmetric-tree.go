/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func recursiveSymmetric(left, right *TreeNode) bool {
    if left == nil && right == nil {
        return true
    } else if left == nil || right == nil {
        return false
    } else if left.Val != right.Val {
        return false
    } else {
        return recursiveSymmetric(left.Left, right.Right) && recursiveSymmetric(left.Right, right.Left)
    }
}

func isSymmetric(root *TreeNode) bool {
    if root == nil {
        return true
    }
    return recursiveSymmetric(root.Left, root.Right)
}
