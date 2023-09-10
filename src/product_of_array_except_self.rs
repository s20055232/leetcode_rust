#[allow(dead_code)]
impl Solution {
    /// 解法：乘積扣除掉自己，就是自己的左邊元素全部相乘然後與右邊元素全部相乘
    /// Time: O(n＾2)，可以通過但不符合題目需求（題目要求O(n)且不能使用除法）
    /// Space: O(1)，沒有用到額外的空間（輸出不包含在計算之中）
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for (idx, _) in nums.iter().enumerate() {
            result.push(
                nums[..idx].iter().product::<i32>() * nums[idx + 1..].iter().product::<i32>(),
            );
        }
        result
    }
    /// 解法：先建立一個全為1的動態array，先從左邊走過一次，將乘積更新到result裡面（這時候result會是除了自己的乘積，因為我們先給定元素為1，所以當迴圈遇到自己時，會直接replace成左邊的乘積），也更新left的乘積。
    /// 接著從右邊走過，將乘積更新到result裡面（這時候result會是左邊的乘積與右邊乘積的相乘，依然不包含自己），也更新right的乘積。
    /// 為什麼要這樣做？ 因為乘積就是將所有元素進行乘法運算，將左邊與右邊乘積相乘就可以得到答案，先將初始array都設為1，就可以讓乘積不受自己的數字影響。
    /// Time: O(n)，走過兩次輸入，O(2n)
    /// Space: O(1)，除了輸出沒用到與輸入大小有關的額外空間
    pub fn product_except_self_2(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];
        let mut left = 1;
        for i in (0..).take(n) {
            result[i] *= left;
            left *= nums[i];
        }
        let mut right = 1;
        for i in (0..n).rev() {
            result[i] *= right;
            right *= nums[i];
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
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(Solution::product_except_self(nums), [24, 12, 8, 6].to_vec());
        let nums = [-1, 1, 0, -3, 3].to_vec();
        assert_eq!(
            Solution::product_except_self(nums),
            [0, 0, 9, 0, 0].to_vec()
        );
    }
    #[test]
    fn test_2() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(
            Solution::product_except_self_2(nums),
            [24, 12, 8, 6].to_vec()
        );
        let nums = [-1, 1, 0, -3, 3].to_vec();
        assert_eq!(
            Solution::product_except_self_2(nums),
            [0, 0, 9, 0, 0].to_vec()
        );
    }
}
