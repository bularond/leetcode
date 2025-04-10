import "sort"

type link struct {
    prev int
    ans  int
}

func largestDivisibleSubset(nums []int) []int {
    sort.Ints(nums)

    links := make([]link, len(nums))
    for i := range links {
        links[i].prev = -1
        links[i].ans  = 1
    }

    longestLinkPos := 0
    for i := range links {
        for j := i + 1; j < len(nums); j++ {
            if nums[j] % nums[i] == 0 {
                if links[i].ans + 1 > links[j].ans {
                    links[j].prev = i
                    links[j].ans = links[i].ans + 1
                }
            }
        }

        if links[i].ans > links[longestLinkPos].ans {
            longestLinkPos = i
        }
    }

    ans := make([]int, 0, links[longestLinkPos].ans)
    for longestLinkPos != -1 {
        ans = append(ans, nums[longestLinkPos])
        longestLinkPos = links[longestLinkPos].prev
    }

    return ans
}