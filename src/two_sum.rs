use std::{cmp::Ordering, collections::HashMap};
#[allow(dead_code)]
impl Solution {
    /// 第一種解法，暴力解，走過每一個元素，查看該元素的差值是否存在於剩下的元素中，找到的話回傳，找不到進行下一輪
    /// Time Complexity: O(n^2)，需要進行兩次迴圈，為n的平方
    /// Space Complexity: O(1)，生成一次固定長度為2的vector，與n無關，因此為常數項
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(2);
        for (idx, i) in nums.iter().enumerate() {
            let rest = target - i;
            for (idx2, j) in nums.iter().enumerate() {
                if idx == idx2 {
                    continue;
                }
                if &rest == j {
                    result.push(idx as i32);
                    result.push(idx2 as i32);
                    return result;
                }
            }
        }
        result
    }
    /// 第二種解法，走過每一個元素，每個元素都去HashMap中搜尋此差值是否存在，如果不存在就儲存差值作為key，索引作為value。
    /// 找到時就是該元素的差值存在於HashMap。
    /// Time Complexity: O(n)，只需走過一次給定的n長度的串列
    /// Space Complexity: O(n)，需要生成一個HashMap紀錄過去的差值，空間與n成正比
    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map = HashMap::new();
        let mut result = Vec::with_capacity(2);
        for (idx, i) in nums.iter().enumerate() {
            let diff = target - i;
            hash_map.entry(diff).or_insert(idx);
            match hash_map.get(i) {
                Some(&x) if x != idx => {
                    result.push(x as i32);
                    result.push(idx as i32);
                    return result;
                }
                _ => continue,
            }
        }
        result
    }
    /// 第三種解法，先將vector排序，用雙指針的方式從頭尾開始遍歷。
    /// 當數值大於0時，右指針索引-1，讓總體變小，反之則左指針+1，讓總體變大。
    /// Time Complexity: O(n logn)，排序需要花上O(n logn)，遍歷需要O(n)，總合是O(n logn + n)
    /// Space Complexity: O(n)，需要生成一個索引表，空間與n成正比
    pub fn two_sum_3(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums.iter().enumerate().collect::<Vec<(usize, &i32)>>();
        nums.sort_unstable_by_key(|k| k.1);
        let mut result = Vec::with_capacity(2);
        let mut start = 0;
        let mut end = nums.len() - 1;
        while result.len() < 2 {
            let diff = target - nums[start].1 - nums[end].1;
            match diff.cmp(&0) {
                Ordering::Equal => {
                    result.push(nums[start].0 as i32);
                    result.push(nums[end].0 as i32);
                    return result;
                }
                Ordering::Greater => start += 1,
                Ordering::Less => end -= 1,
            };
        }
        result
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test() {
        let result = Solution::two_sum(Vec::from([2, 7, 11, 15]), 9);
        assert_eq!(result, [0, 1]);
        let result = Solution::two_sum(Vec::from([3, 2, 4]), 6);
        assert_eq!(result, [1, 2]);
        let result = Solution::two_sum(Vec::from([3, 3]), 6);
        assert_eq!(result, [0, 1]);
    }
    #[test]
    fn test_2() {
        let result = Solution::two_sum_2(Vec::from([2, 7, 11, 15]), 9);
        assert_eq!(result, [0, 1]);
        let result = Solution::two_sum_2(Vec::from([3, 2, 4]), 6);
        assert_eq!(result, [1, 2]);
        let result = Solution::two_sum_2(Vec::from([3, 3]), 6);
        assert_eq!(result, [0, 1]);
    }
    #[test]
    fn test_3() {
        let result = Solution::two_sum_3(Vec::from([2, 7, 11, 15]), 9);
        assert_eq!(result, [0, 1]);
        let result = Solution::two_sum_3(Vec::from([3, 2, 4]), 6);
        assert_eq!(result, [1, 2]);
        let result = Solution::two_sum_3(Vec::from([3, 3]), 6);
        assert_eq!(result, [0, 1]);
    }
}
