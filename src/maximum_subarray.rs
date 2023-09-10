#[allow(dead_code)]
impl Solution {
    /// 解法：使用滑動視窗，當總和比0還要大的時候，擴大視窗，當總合比0小的時候，縮小視窗，尋找下一個子array
    /// 在每次視窗擴大時，都會確認是否有找到更大的總和
    /// 為何是更新條件是0？ 因為是找最大總和，所以小於0的時候，就可以不用再找了，不如重新開始
    /// Time: O(n)，頂多走過兩次輸入的長度，O(2n)
    /// Space: O(1)，不會使用到額外的空間
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, 0);
        let (mut window_sum, mut max_sum) = (0, i32::MIN);
        while right < nums.len() {
            window_sum += nums[right];
            right += 1;
            max_sum = window_sum.max(max_sum);
            while window_sum < 0 {
                window_sum -= nums[left];
                left += 1;
            }
        }
        max_sum
    }
    /// 解法：走過輸入並累積加總，每次都會比較此輪的加總是否比過去都大，當遇到總和小於0時，從0開始重新累積
    /// Time: O(n)，走過輸入一次
    /// Space: O(1)，使用空間不會與輸入有關
    pub fn max_sub_array_2(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut current = 0;
        for n in nums {
            current += n;
            max_sum = max_sum.max(current);
            if current < 0 {
                current = 0;
            }
        }
        max_sum
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4].to_vec();
        assert_eq!(Solution::max_sub_array(nums), 6);
        let nums = [1].to_vec();
        assert_eq!(Solution::max_sub_array(nums), 1);
        let nums = [5, 4, -1, 7, 8].to_vec();
        assert_eq!(Solution::max_sub_array(nums), 23);
    }
    #[test]
    fn test_2() {
        let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4].to_vec();
        assert_eq!(Solution::max_sub_array_2(nums), 6);
        let nums = [1].to_vec();
        assert_eq!(Solution::max_sub_array_2(nums), 1);
        let nums = [5, 4, -1, 7, 8].to_vec();
        assert_eq!(Solution::max_sub_array_2(nums), 23);
    }
}
