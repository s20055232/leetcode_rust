use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    /// 解法：先計算字串1所有字元出現次數，然後在查看字串2將次數減去，如果都為0則正確
    /// Time: O(n)，走過兩組字串
    /// Space: O(n)，會隨著字串長度變長
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut letter_s = HashMap::new();
        for letter in s.chars() {
            letter_s
                .entry(letter)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for letter in t.chars() {
            if let Some(x) = letter_s.get_mut(&letter) {
                *x -= 1;
                if *x < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
    /// 解法：跟上面的解法一樣，但嘗試減少code的數量
    pub fn is_anagram_2(s: String, t: String) -> bool {
        let mut letter = HashMap::new();
        s.chars().for_each(|c| {
            letter
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });
        t.chars().for_each(|c| {
            if let Some(x) = letter.get_mut(&c) {
                *x -= 1;
            }
        });
        letter.values().all(|x| x == &0)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert!(Solution::is_anagram(s, t));
        let s = String::from("rat");
        let t = String::from("cat");
        assert!(!Solution::is_anagram(s, t));
    }
    #[test]
    fn test_2() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert!(Solution::is_anagram_2(s, t));
        let s = String::from("rat");
        let t = String::from("cat");
        assert!(!Solution::is_anagram_2(s, t));
    }
}
