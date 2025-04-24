func removeDuplicates(nums []int) int {
    l := 0
    r := 0
    for r < len(nums) {
        if nums[l] != nums[r] {
            l++
            nums[l] = nums[r]
        }
        r++
    }

    return l + 1
}

/*
    r
  l
0,1,1,1,1,2,2,3,3,4
*/