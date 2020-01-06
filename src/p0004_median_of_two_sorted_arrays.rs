pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        let left = (m + n + 1) / 2;
        let right = (m + n + 2) / 2;

        (
            Solution::find(&nums1, 0, &nums2, 0, left)
            + Solution::find(&nums1, 0, &nums2, 0, right)
        ) / 2.0
    }

    fn find(nums1: &Vec<i32>, i: usize, nums2: &Vec<i32>, j: usize, k: usize) -> f64 {
        fn min(num1: i32, num2: i32) -> i32 {
            if num1 < num2 { num1 } else { num2 }
        }

        if i >= nums1.len() {
            return nums2[j + k - 1] as f64;
        }

        if j >= nums2.len() {
            return nums1[j + k - 1] as f64;
        }

        if k == 1 {
            return min(nums1[i], nums2[j]) as f64;
        }

        let idx1 = i + k / 2 - 1;
        let idx2 = j + k / 2 - 1;
        let mid1 = if idx1 < nums1.len() { nums1[idx1] } else { std::i32::MAX };
        let mid2 = if idx2 < nums2.len() { nums2[idx2] } else { std::i32::MAX };

        if mid1 < mid2 {
            Solution::find(nums1, i + k / 2, nums2, j, k - k / 2)
        } else {
            Solution::find(nums1, i, nums2, j + k / 2, k - k / 2)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1], vec![2]),
            1.5
        );
        
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![], vec![1, 2]),
            1.5
        );
        
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![]),
            2.0
        );
        
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![2, 4]),
            2.0
        );
    }
}