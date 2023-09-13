#[allow(dead_code)]
impl Solution {
    /// 解法：先將數值依照每個區間的最小值排序，這樣排序就可以兩兩比較，且數值會循序漸進
    /// 排序後，開始遍歷，第一輪不用比較，直接將vector放進去。
    /// 第二輪開始，開始查看是否有數值區間重疊，有兩種情況，一種是沒有重疊，也就是上一輪的最大值比這一輪的最小值小，代表沒有重疊，可以直接將vector放進去
    /// 如果有重疊，則查看最大值要放誰（我們不用決定最小值放誰，因爲排序時已經確定最小值會是上一輪的了），並更新上一輪的上限區間
    /// Time: O(n logN)，排序加上遍歷一次元素是O(n logN + n)
    /// Space: O(logN)，排序需要使用O(logN)的空間
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 小小大大
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|v| v[0]);
        let mut result: Vec<Vec<i32>> = Vec::new();
        for interval in intervals {
            // 如果是第一輪，result為空，直接放進去，或是上一輪的結果的最大值與這輪的最小值沒有重疊，也直接放進去
            if result.is_empty() || result.last_mut().unwrap()[1] < interval[0] {
                result.push(interval);
            } else {
                // 如果有重疊，取雙邊最大，看哪比較大，更新最大區間數值
                let last_interval = result.last_mut().unwrap();
                last_interval[1] = last_interval[1].max(interval[1]);
            }
        }
        result
    }
    /// 解法：跟上一輪邏輯一樣，但用了更多函數式的風格。fold這個方法讓使用者輸入初始狀態，並執行給予的函數，函數回傳的數值將作為下一輪迭代的參數。
    /// 這邊將初始的區間作為初始狀態傳入，如果沒有重疊，代表沒得比，將區間塞入result，如果有重疊，代表這輪區間還有得比，傳入下一輪作為參數。
    /// 最後將比到最後的區間塞入result，就可以得到結果了
    /// Time 跟 Space都跟上面的解法一樣
    pub fn merge_2(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|v| v[0]);
        let mut result: Vec<Vec<i32>> = Vec::new();
        let last = intervals
            .iter()
            .fold((intervals[0][0], intervals[0][1]), |(l, r), current| {
                if current[0] > r {
                    // 沒得比的塞進去result
                    result.push(vec![l, r]);
                    return (current[0], current[1]);
                }
                // 有得比的傳給下一輪
                (l, current[1].max(r))
            });
        // 最後將比到最後的塞進去
        result.push(vec![last.0, last.1]);
        result
    }
}
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let intervals = [vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]].to_vec();
        assert_eq!(
            Solution::merge(intervals),
            [[1, 6].to_vec(), [8, 10].to_vec(), [15, 18].to_vec()].to_vec()
        );
        let intervals = [vec![1, 4], vec![4, 5]].to_vec();
        assert_eq!(Solution::merge(intervals), vec![vec![1, 5]]);
    }
    #[test]
    fn test_2() {
        let intervals = [vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]].to_vec();
        assert_eq!(
            Solution::merge_2(intervals),
            [[1, 6].to_vec(), [8, 10].to_vec(), [15, 18].to_vec()].to_vec()
        );
        let intervals = [vec![1, 4], vec![4, 5]].to_vec();
        assert_eq!(Solution::merge_2(intervals), vec![vec![1, 5]]);
    }
}
