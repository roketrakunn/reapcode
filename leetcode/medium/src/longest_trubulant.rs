
struct Solution; 

use std::{cmp::Ordering};
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
           
        let mut curr = 1 ; 
        let mut  best = 1; 
        let mut prev = Ordering::Equal;
        for i in 1..arr.len() { 

            let sign = arr[i].cmp(&arr[i-1]); 

            match sign {
                Ordering::Equal => curr = 1,
                s if s == prev => curr =2,
                _               => curr +=1,
            }
            best = best.max(curr);
            prev = sign
        }
        best
    }
}
