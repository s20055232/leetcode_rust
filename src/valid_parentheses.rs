use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    /// 解法：遇到左邊的就push進去，遇到右邊的就push出來匹配，使用HashMap可以在左右成對元素很多的時候也很快
    /// Time: O(n)
    /// Space: O(1)
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let map = [('}', '{'), (']', '['), (')', '(')]
            .iter()
            .cloned()
            .collect::<HashMap<char, char>>();
        for i in s.chars() {
            match i {
                '{' | '(' | '[' => stack.push(i),
                '}' | ')' | ']' => {
                    if map.get(&i) != stack.pop().as_ref() {
                        return false;
                    }
                }
                _ => continue,
            }
        }
        stack.is_empty()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let s = "()";
        assert!(Solution::is_valid(s.to_string()));
        let s = "()[]{}";
        assert!(Solution::is_valid(s.to_string()));
        let s = "(]";
        assert!(!Solution::is_valid(s.to_string()));
    }
}
