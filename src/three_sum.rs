use std::cmp::Ordering;

#[allow(dead_code)]
impl Solution {
    /// 解法：先進行排序，運用雙指針技巧，分別從頭尾遍歷。
    /// 步驟1: 迴圈走過輸入，設定頭指針索引下一個元素，尾指針指向尾巴的元素，這麼做是因為我們排序過，只需要看後面的元素，之前的都不用看，看了只會得到重複的結果。
    /// 步驟2: 頭尾指針開始移動，如果總和比0小，移動左指針，反之移動右指針，遇到總和為0時，將結果加到輸出
    /// 步驟3: 找到可行結果之後要避免重複，將指針往前移動，避免找到一樣的解答
    /// Time: O(n^2)，排序O(n logN)加上巢狀遍歷元素O(n^2)是O(N^2 + nlogN)
    /// Space: O(1)，除了輸出以外，沒有受輸入影響的額外空間
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        let mut result: Vec<Vec<i32>> = vec![];
        nums.sort_unstable();
        let length = nums.len();
        for i in 0..length {
            // 跟上一輪一樣的跳過，不需要重複計算
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = length - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                match sum.cmp(&0) {
                    Ordering::Greater => right -= 1,
                    Ordering::Less => left += 1,
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        // 這組合可行，但不能重複，遇到重複元素開始跳過
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        left += 1;
                        right -= 1;
                    }
                }
            }
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
        let arr = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(
            Solution::three_sum(arr),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        let nums = vec![0, 1, 1];
        let answer: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(nums), answer);

        let nums = [0, 0, 0].to_vec();
        assert_eq!(Solution::three_sum(nums), [[0, 0, 0].to_vec()].to_vec());

        let nums = [1, -1, -1, 0].to_vec();
        assert_eq!(Solution::three_sum(nums), [[-1, 0, 1].to_vec()].to_vec());
    }
}
