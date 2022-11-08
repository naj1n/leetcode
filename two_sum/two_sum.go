// Time complexity: O(n)
// Space complexity: O(n)
func twoSum(nums []int, target int) []int {
	hashmap := make(map[int]int)

	for index, num := range nums {
		if prevIndex, exists := hashmap[target - num]; exists {
			return  []int{prevIndex, index}
		}
		hashmap[num] = index
	}

	return []int{}
}
