import "math"

func minimumTotal(triangle [][]int) int {
    for level, line := range triangle {
        if level == 0 {
            continue
        }

        prevLine := triangle[level - 1]
        for pos, val := range line {
            left := math.MaxInt
            right := math.MaxInt
            if pos < len(prevLine) {
                right = prevLine[pos] + val
            }
            if pos != 0 {
                left =  prevLine[pos-1] + val
            }

            if left < right {
                line[pos] = left
            } else {
                line[pos] = right
            }
        }
    }

    lastLine := triangle[len(triangle) - 1]
    ans := math.MaxInt
    for _, val := range lastLine {
        if val < ans {
            ans = val
        }
    }

    return ans
}

/*
[2],
[3,4],
[6,5,7],
[4,1,8,3]
*/