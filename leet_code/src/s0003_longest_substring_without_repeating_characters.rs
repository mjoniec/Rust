pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let seq: Vec<char> = s.chars().collect();
        let (mut start, mut end, mut max) = (0, 0, 0);

        while end < seq.len() {
            for i in start..end {
                if seq[end] == seq[i] {
                    start = i + 1;
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

    //O(n) solution
    pub fn length_of_longest_substring_my_version(s: String) -> i32 {
        let seq: Vec<char> = s.chars().collect();
        let mut max: usize = 0;//maximum size of sliding window
        
        //sliding window left and right pointers - storing indexes from 0
        let mut left: usize = 0;
        let mut right: usize = 0;

        //array to store locations per each letter - if same letter occurs multiple times then last occurence stored
        //value - location of last detected letter in string - counted from 1 - if 0 - letter not detected by right pointer yet - so position = index + 1
        //index - ascii code of each letter serves as an established index - speeds up retreival to O(1)
        let mut letters_positions: [usize;128] = [0;128];

        while right < seq.len() {
            let current_letter_ascii = seq[right] as usize;

            //println!("_______________");
            //println!("current letter: {:?} - ascii: {:?}", seq[right], current_letter_ascii);
            
            //trick to this question is finding out where to pick next substring beginnin (left pointer) once duplicate detected
            //previous ocurence of letter pointed by right existed >> we need to move left pointer to current letter previously detected position + 1 (counting from 1, 0 for non detected letters)
            if letters_positions[current_letter_ascii] != 0 
                && left < letters_positions[current_letter_ascii]{ //we can only move pointers to the right side, replace pointers with larger numers
                // println!("for letter: {:?} previous occurence detected at position: {:?} 
                //     so replacing left pointer {:?} with it as index (covering +1)", seq[right], letters_positions[current_letter_ascii], left);

                left = letters_positions[current_letter_ascii];//+1 covered by the fact that we count letters from 1
            }

            //new max length of sliding window
            if max < right - left + 1{
                max = right - left + 1;
            }

            //println!("left: {:?} right: {:?} max: {:?}", left, right, max);

            //unconditional operations each loop:
            // - assign current letter position to indexes
            // - move right pointer 
            right += 1;
            letters_positions[current_letter_ascii] = right;//current letter occurence assigned unconditionally, +1 for position instead of index covered above            
        }

        // println!("{:?}", letters_positions);
        // println!("max: {:?}", max);

        max as i32
    }
}

pub fn run() {
    println!("testing s0003_longest_substring_without_repeating_characters");
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);

    assert_eq!(Solution::length_of_longest_substring_my_version("abcabcbb".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring_my_version("bbbb".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring_my_version("pwwkew".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring_my_version("abba".to_string()), 2);
}
