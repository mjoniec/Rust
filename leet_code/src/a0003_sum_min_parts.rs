
pub struct Solution {}
impl Solution {
    pub fn sum_min_parts(mut v: Vec<i32>) -> i32 {
        let mut sum = 0;

        while v.len() > 1 {
            v.sort();
            let min1 = v.remove(0);
            let min2 = v.remove(0);
            let new_sum = min1 + min2;
            sum += new_sum;
            v.push(new_sum);
        }

        //println!("sum: {:?}", sum);
        sum
    }
}

pub fn run() {
    println!("testing a0003_sum_min_parts");
    assert_eq!(Solution::sum_min_parts(vec![8, 4, 6, 12]), 58);
    assert_eq!(Solution::sum_min_parts(vec![20, 4, 8, 2]), 54);
    assert_eq!(Solution::sum_min_parts(vec![1, 2, 5, 10, 35, 89]), 224);
}
