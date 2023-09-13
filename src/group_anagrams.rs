use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    /// 解法：將每個元素排序，排序後的元素若一致則代表可以做易位構詞，作為HashMap的key，排序前的原始元素是其value
    /// Time: O(N logN)，遍歷vector是O(N)並在裡面做排序是O(K logK)，因此是O(N * K logK)
    /// Space: O(N)，最糟的情況下，HashMap的key都不會重複
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hashmap = HashMap::new();
        let mut strs = strs;
        for i in strs.iter_mut() {
            let mut tmp = i.as_bytes().to_vec();
            tmp.sort_unstable();
            hashmap.entry(tmp).or_insert_with(Vec::new).push(i.clone());
        }

        hashmap.into_values().collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>();

        assert_eq!(
            Solution::group_anagrams(strs),
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        );

        let strs = [""].iter().map(|&x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(Solution::group_anagrams(strs), vec![vec![""]]);

        let strs = ["a"]
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>();
        assert_eq!(Solution::group_anagrams(strs), vec![vec!["a"]]);
    }
}
