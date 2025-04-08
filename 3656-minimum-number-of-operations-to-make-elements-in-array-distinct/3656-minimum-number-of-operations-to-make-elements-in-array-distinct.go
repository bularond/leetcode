func minimumOperations(nums []int) int {
	counts := make(map[int]int)
	countNonDistinct := 0
	for _, val := range nums {
		count, ok := counts[val]
		if !ok {
			counts[val] = 1
		} else {
			if count == 1 {
				countNonDistinct++
			}
			counts[val] = count + 1
		}
	}
  
	ans := 0
	pos := 0
	for countNonDistinct > 0 && pos < len(nums) {
		if pos%3 == 0 {
			ans++
		}

		val := nums[pos]
		count := counts[val]
		if count == 2 {
			countNonDistinct--
		}
		counts[val] = count - 1

		pos++
	}

	return ans
}