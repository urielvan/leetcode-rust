pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();

        if len < 2 {
            return len as i32;
        }

        let (mut start, mut end, mut max) = (0, 1, 1);

        while end < len {
            for idx in start..end {
                if seq[end] == seq[idx] {
                    start = idx + 1;
                    break;
                }
            }
            let curr = end - start + 1;
            if curr > max {
                max = curr
            }
            end += 1
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("")),
            0
        );
    }
}
