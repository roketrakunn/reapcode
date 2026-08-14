use std::collections::HashSet;

struct  Solution;

impl Solution {
    pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
        // OR only ever turns bits on, so a subsequence ORs to exactly 2^k
        // only if every element is 2^k itself -> 2^k expressible <=> 2^k in nums.
        //
        // and the answer must be a power of two: if x had two set bits 2^a | 2^b,
        // both are smaller than x, so both would be expressible, so both are in
        // nums, so OR-ing them gives x. contradiction.
        let seen: HashSet<i32> = nums.into_iter().collect();

        // nums[i] <= 10^9 < 2^30, so bits 0..29 are the only ones available
        (0..30)
            .map(|k| 1 << k)
            .find(|p| !seen.contains(p))
            .unwrap_or(1 << 30)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Solution::min_impossible_or(vec![2, 1]), 4);
        assert_eq!(Solution::min_impossible_or(vec![5, 3, 2]), 1);
    }

    #[test]
    fn all_bits_present() {
        let all: Vec<i32> = (0..30).map(|k| 1 << k).collect();
        assert_eq!(Solution::min_impossible_or(all), 1 << 30);
    }
}
