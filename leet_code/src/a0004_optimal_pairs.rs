//optimal sum of 1 element from each list as close to optimal sum as possible
//once best pair is choosen, search for another pairs that equal that number and return them all
//if some pair would sum less do not include it - leftover elements
//element is a vector - first element is id another is value - value is to be added
use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn optimal_pairs(optimal_sum: i32, vec1: Vec<Vec<i32>>, vec2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        let mut v_out: Vec<Vec<i32>> = Vec::new();

        for v in vec1{
            map1.insert(v[0], v[1]);
        }
        for v in vec2{
            map2.insert(v[0], v[1]);
        }

        let mut key1: i32 = 0;
        let mut key2: i32 = 0;
        let mut candidate_sum: i32 = 0;

        // for(k1, v1) in map1{
        //     for(k2, v2) in map2{
        //         let sum = v1 + v2;
        //         if sum > candidate_sum && sum <= optimal_sum {
        //             candidate_sum = sum;
        //             key1 = k1;
        //             key2 = k2;
        //             if sum == optimal_sum {
        //                 break;
        //             }
        //         }
        //     }
        // }

        v_out.push(vec![key1, key2]);//println!("sum: {:?}", sum);
        
        v_out
    }
}

pub fn run() {
    println!("testing a0004_optimal_pairs off");
    //assert_eq!(Solution::optimal_pairs(7, vec![vec![1, 2], vec![1, 2]], vec![vec![1, 2], vec![1, 2]]), vec![vec![1, 2], vec![1, 2]]);
    //let output = Solution::optimal_pairs(7, vec![vec![1, 2], vec![1, 2]], vec![vec![1, 2], vec![1, 2]]);

    println!("vec: {:?}", output);

}
