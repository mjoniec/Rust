//You are given a string s and an integer k.
//You can choose any character of the string and change it to any other uppercase English character. 
//You can perform this operation at most k times.
//Return the length of the longest substring containing the same letter you can get after performing the above operations.
pub struct Solution {}
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut max: usize = 0;//maximum size of sliding window that met criteria
        let mut left: usize = 0; //sliding window left and right pointers - storing indexes from 0
        let mut right: usize = 0;
        let mut letters_counter: [usize;128] = [0;128]; //store number of occurences of each letter inside sliding window CURRENTLY in loop

        while right < s_chars.len() {
            letters_counter[s_chars[right] as usize] += 1;//increment letter pointed by right
            let current_max_letter_occurences = letters_counter.iter().max().unwrap();

            if *current_max_letter_occurences + (k as usize) < right - left + 1{//impossible to have the same letter in window - we need to shrink it
                //println!("overflow at left: {:?} right: {:?} k: {:?} current max letter: {:?}", left, right, k, *current_max_letter_occurences);
                letters_counter[s_chars[left] as usize] -= 1;
                left += 1;
            }

            if max < right - left + 1{ 
                max = right - left + 1; 
            }

            //println!("current left: {:?} current right: {:?} current max: {:?}", left, right, max);
            right += 1; //unconditional right pointer move
        }

        max as i32
    }
}

pub fn run() {
    println!("testing s0424_longest_repeating_character_replacement");
    assert_eq!(Solution::character_replacement("BAAAB".to_string(), 2), 5);
    assert_eq!(Solution::character_replacement("ABB".to_string(), 2), 3);
    assert_eq!(Solution::character_replacement("ABBAA".to_string(), 2), 5);
    assert_eq!(Solution::character_replacement("ABCCCCABD".to_string(), 2), 6);
    assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
}
