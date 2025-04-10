func minOperations(nums []int, k int) int {
    set := make(map[int]struct{})
    for _, num := range nums {
        if num < k {
            return -1
        }
        set[num] = struct{}{}
    }

    ans := len(set)
    if _, ok := set[k]; ok {
        ans--
    }

    return ans
}