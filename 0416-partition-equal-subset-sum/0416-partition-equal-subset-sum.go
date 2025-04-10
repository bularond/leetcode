func canPartition(nums []int) bool {
    sum := 0
    for _, num := range nums {
        sum += num
    }

    if sum % 2 == 1 {
        return false
    }

    sum /= 2
    backpack := make([]bool, sum + 1)
    backpack[0] = true

    for _, num := range nums {
        for i := sum - num; i >= 0; i-- {
            if backpack[i] {
                backpack[i + num] = true
            }
        }
        if backpack[sum] {
            return true
        }
    }

    return false
}
/*
0 1 2 3 4
t 1 f f f
*/