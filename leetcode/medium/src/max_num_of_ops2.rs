
struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        fn count(nums: &Vec<i32>, l: usize, r: usize, score: i32, memo: &mut HashMap<(usize, usize), i32>) -> i32 {
            if l >= r {
                return 0;
            }
            if let Some(&cached) = memo.get(&(l, r)) {
                return cached;
            }

            let mut best = 0;

            if l < r && nums[l] + nums[l + 1] == score {
                best = best.max(1 + count(nums, l + 2, r, score, memo));
            }
            if r > 1 && nums[r] + nums[r - 1] == score {
                best = best.max(1 + count(nums, l, r - 2, score, memo));
            }
            if nums[l] + nums[r] == score {
                best = best.max(1 + count(nums, l + 1, r - 1, score, memo));
            }

            memo.insert((l, r), best);
            best
        }

        let s1 = count(&nums, 0, n - 1, nums[0] + nums[1], &mut HashMap::new());
        let s2 = count(&nums, 0, n - 1, nums[n - 1] + nums[n - 2], &mut HashMap::new());
        let s3 = count(&nums, 0, n - 1, nums[0] + nums[n - 1], &mut HashMap::new());

        s1.max(s2).max(s3)
    }
}
