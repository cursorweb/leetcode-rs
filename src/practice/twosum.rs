// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, idx) in nums.iter().zip(0..nums.len()) {
        let i = *i;
        let idx = idx as i32;
        let diff = target - i;

        let compi = nums.iter().position(|x| *x == diff);

        if let Some(compi) = compi {
            let compi = compi as i32;
            if compi == idx {
                continue;
            }

            return vec![idx, compi];
        }
    }
    
    vec![]
}