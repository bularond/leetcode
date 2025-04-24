func climbStairs(n int) int {
    arr := make([]int, n + 2)
    arr[0] = 1
    for i := 0; i < n; i++ {
        arr[i + 1] += arr[i]
        arr[i + 2] += arr[i]
    }

    return arr[n]
}
