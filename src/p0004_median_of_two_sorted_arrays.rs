pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        let mid_idx = (m + n - 1) / 2;

        let (mut i, mut j) = (0, 0);
        let (mut ans, mut cur_num) = (0.0, 0);
        while i < m || j < n {
            if j == n || (i < m && nums1[i] <= nums2[j]) { cur_num = nums1[i]; }
            else { cur_num = nums2[j]; };

            if i + j >= mid_idx && i + j <= mid_idx + 1 {
                if (m + n) % 2 == 1 {
                    return cur_num as f64;
                } else {
                    ans += cur_num as f64;
                    if i + j == mid_idx + 1 { return ans / 2.0; }
                }
            }

            if j == n || (i < m && nums1[i] <= nums2[j]) { i += 1; }
            else { j += 1 };
        }

        ans
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
        
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]),
            0.0
        );
        
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![1, 2]),
            1.5
        );
    }
}