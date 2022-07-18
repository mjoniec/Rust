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

    pub fn merge_my_submission(
        nums1: &mut Vec<i32>, 
        m: i32, 
        nums2: &mut Vec<i32>, 
        n: i32) {
        
        let mut im = m - 1;
        let mut jn = n - 1;
        let mut max = m + n - 1;
        
        while max >= 0 {
            if im >= 0 && (jn < 0 || //if second array already used up we take from 1st by default
                nums1[im as usize] >= nums2[jn as usize]) {
                nums1[max as usize] = nums1[im as usize];
                im -= 1;
            } 
            else {
                nums1[max as usize] = nums2[jn as usize];
                jn -= 1;
            }

            max -= 1;
        }
    }

    pub fn merge_experiments_version(
        nums1: &mut Vec<i32>, 
        m: i32, 
        nums2: &mut Vec<i32>, 
        n: i32) {//para/body separation problem
        
        let mut im = m - 1;
        let mut jn = n - 1;
        let mut max = m + n - 1;
        
        while max >= 0 {
            
            //complex if problem
            let pick_from_nums1 = im >= 0  //if 1st array used up we take from 2nd
                && (jn < 0 ||               //if 2nd array used up we take from 1st by default
                    nums1[im as usize] >= nums2[jn as usize]);
            
            if pick_from_nums1 {
                Self::insert(nums1, nums1[im as usize], max as usize, &mut im);
            } 
            else {
                Self::insert(nums1, nums2[jn as usize], max as usize, &mut jn);
            }
            max -= 1;
        }
    }

    pub fn insert(nums: &mut Vec<i32>, value_to_insert: i32, index: usize, index_for_decrementation: &mut i32) {
        nums[index] = value_to_insert;
        *index_for_decrementation -= 1;
    }
}

pub fn run() {
    println!("testing s0088_merge_sorted_array");

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

    let mut vec1 = vec![1, 2, 3, 0, 0, 0];
    let mut vec2 = vec![2, 5, 6];
    Solution::merge_my_submission(&mut vec1, 3, &mut vec2, 3);
    assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);

    let mut vec1 = vec![1, 2, 3, 0, 0, 0];
    let mut vec2 = vec![2, 5, 6];
    Solution::merge_experiments_version(&mut vec1, 3, &mut vec2, 3);
    assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);
}