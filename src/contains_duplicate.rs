use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    /// 解法：將輸入vector使用HashSet去掉重複，並查看與原始vector的長度差異，如果有重複則長度會不一致
    /// Time: O(n logN)
    /// Space: O(1)
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // 解題思路：先將vector排序，然後遍歷。設個暫存變數用來記錄上一個遇到的元素。
        let mut nums = nums.clone();
        nums.sort_unstable();
        for n in nums.windows(2) {
            if n[0] == n[1] {
                return true;
            }
        }
        false
    }
    /// 解法：將輸入vector使用HashSet去掉重複，並查看與原始vector的長度差異，如果有重複則長度會不一致
    /// Time: O(n)
    /// Space: O(1)
    pub fn contains_duplicate_2(nums: Vec<i32>) -> bool {
        HashSet::<i32>::from_iter(nums.clone()).len() != nums.len()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let array = vec![1, 2, 3, 1];
        assert!(Solution::contains_duplicate(array));
        let array = vec![1, 2, 3, 4];
        assert!(!Solution::contains_duplicate(array));
        let array = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert!(Solution::contains_duplicate(array));
    }

    #[test]
    fn test_1() {
        let array = vec![1, 2, 3, 1];
        assert!(Solution::contains_duplicate_2(array));
        let array = vec![1, 2, 3, 4];
        assert!(!Solution::contains_duplicate_2(array));
        let array = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert!(Solution::contains_duplicate_2(array));
    }
}
