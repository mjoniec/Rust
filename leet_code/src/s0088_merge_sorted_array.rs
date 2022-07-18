pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut z = m + n - 1;
        while z >= 0 {
            if i >= 0 && (j < 0 || nums1[i as usize] >= nums2[j as usize]) {
                nums1[z as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[z as usize] = nums2[j as usize];
                j -= 1;
            }
            z -= 1;
        }
    }
}

pub fn run() {
    println!("testing");

    let mut vec1 = vec![1, 2, 3, 0, 0, 0];
    let mut vec2 = vec![2, 5, 6];
    Solution::merge(&mut vec1, 3, &mut vec2, 3);
    assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);

    let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![];
    Solution::merge(&mut vec1, 3, &mut vec2, 0);
    assert_eq!(vec1, vec![1, 2, 3]);

    let mut vec1 = vec![0, 0, 0];
    let mut vec2 = vec![1, 2, 3];
    Solution::merge(&mut vec1, 0, &mut vec2, 3);
    assert_eq!(vec1, vec![1, 2, 3]);
}