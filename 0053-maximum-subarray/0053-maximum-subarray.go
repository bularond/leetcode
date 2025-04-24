func maxSubArray(nums []int) int {
    prefixSum := 0
    lowestPrefixSum := 0
    ans := nums[0]

    for _, val := range nums {
        prefixSum += val
        if prefixSum - lowestPrefixSum > ans {
            ans = prefixSum - lowestPrefixSum
        }
        if prefixSum < lowestPrefixSum {
            lowestPrefixSum = prefixSum
        }
    }

    return ans
}
