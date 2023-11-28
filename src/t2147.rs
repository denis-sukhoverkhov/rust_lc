// 2147. Number of Ways to Divide a Long Corridor
// https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor/

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut ct_chairs = 0;
        let mut last_chair_index = -1;

        let mut ans: i64 = 1;

        for (i, c) in corridor.chars().enumerate() {
            if c == 'P' {
                continue;
            }
            ct_chairs += 1;
            if ct_chairs == 3 {
                let bars = i as i32 - last_chair_index;
                ans = (ans * bars as i64) % 1_000_000_007;
                ct_chairs = 1;
            }
            last_chair_index = i as i32;
        }

        if ct_chairs < 2 {
            return 0;
        }

        ans as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_ways_0_chairs() {
        assert_eq!(Solution::number_of_ways("P".to_string()), 0);
    }

    #[test]
    fn test_number_of_ways_1_chair() {
        assert_eq!(Solution::number_of_ways("S".to_string()), 0);
    }

    #[test]
    fn test_number_of_ways_2_chairs() {
        assert_eq!(Solution::number_of_ways("PPSPSP".to_string()), 1);
    }

    #[test]
    fn test_number_of_ways_4_chairs() {
        assert_eq!(Solution::number_of_ways("SSPPSPS".to_string()), 3);
    }

    #[test]
    fn test_number_of_ways_ex1() {
        assert_eq!(Solution::number_of_ways("SPSPPSSPSSSS".to_string()), 6);
    }

    #[test]
    fn test_number_of_ways_ex2() {
        let corridor = "PPPPPSPPSPPSPPPSPPPPSPPPPSPPPPSPPSPPPSPSPPPSPSPPPSPSPPPSPSPPPPSPPPPSPPPSPPSPPPPSPSPPPPSPSPPPPSPSPPPSPPSPPPPSPSPSS".to_string();
        assert_eq!(Solution::number_of_ways(corridor), 919999993);
    }

    #[test]
    fn test_number_of_ways_ex3() {
        let corridor = "PPPPPPPSPPPSPPPPSPPPSPPPPPSPPPSPPSPPSPPPPPSPSPPPPPSPPSPPPPPSPPSPPSPPPSPPPPSPPPPSPPPPPSPSPPPPSPSPPPSPPPPSPPPPPSPSPPSPPPPSPPSPPSPPSPPPSPPSPSPPSSSS".to_string();
        assert_eq!(Solution::number_of_ways(corridor), 18335643);
    }
}
