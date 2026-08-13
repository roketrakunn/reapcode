struct Solution;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut dp = energy;

        let n = dp.len();
        let k = k as usize;

        // walk backwards so dp[i + k] is already the full chain from i + k onwards
        for i in (0..n.saturating_sub(k)).rev() {
            dp[i] += dp[i + k];
        }

        Solution::get_max(dp)
    }

    fn get_max(arr : Vec<i32>) -> i32 {

        let mut  maxi = i32::MIN; // energy can be negative

        for val in arr.iter() {
            maxi = maxi.max(*val);
        }
        maxi
    }
}
