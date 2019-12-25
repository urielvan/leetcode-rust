use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let length = nums.len();
    let mut rest: i32;
    let mut hash: HashMap<&i32, i32> = HashMap::with_capacity(length);

    for i in 0..length {
        rest = target - nums[i];

        if hash.contains_key(&rest) {
            return vec![hash[&rest] as i32, i as i32];
        } else {
            hash.insert(&nums[i], i as i32);
        }
    }

    vec![]
}

#[cfg(test)]
mod test {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![1, 2, 3, 7], 9), vec![1, 3]);

        assert_eq!(two_sum(vec![1], 9), vec![]);
    }
}
