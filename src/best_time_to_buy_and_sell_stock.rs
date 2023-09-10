#[allow(dead_code)]
impl Solution {
    /// 第一種解法，暴力解，走過所有元素，來查看哪一個差值最大。
    /// Time: O(n^2)，需要巢狀迴圈
    /// Space: O(1)，沒有與輸入長度一起增長的資料
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        for (idx, i) in prices.iter().enumerate() {
            for j in &prices[idx..] {
                if j - i > max {
                    max = j - i;
                }
            }
        }
        max
    }
    /// 第二種解法，看到比自己小的點就移動過去當作起點，看到比較大的就紀錄差值，有更多獲利就登記為最大值
    /// Time: O(n)，只需走過一次vector
    /// Space: O(1)，沒有與輸入長度一起增長的資料
    pub fn max_profit_2(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut buy_price = prices[0];
        for p in &prices[1..] {
            let diff = p - buy_price;
            if diff > profit {
                profit = diff;
            } else if diff < 0 {
                buy_price = *p;
            }
        }
        profit
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(prices), 5);
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0);
    }
    #[test]
    fn test_2() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit_2(prices), 5);
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit_2(prices), 0);
    }
}
