// 1235. Maximum Profit in Job Scheduling
// https://leetcode.com/problems/maximum-profit-in-job-scheduling

#[derive(Default)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = start_time
            .iter()
            .zip(end_time.iter())
            .zip(profit.iter())
            .map(|((s, e), p)| (s, e, p))
            .collect::<Vec<_>>();

        jobs.sort_by(|a, b| a.0.cmp(b.0));

        let len_jobs = jobs.len();

        struct Dfs<'s> {
            f: &'s dyn Fn(&mut Dfs, usize) -> i32,
            cache: Vec<i32>,
        }

        let mut dfs = Dfs {
            cache: vec![-1; len_jobs],
            f: &|dfs, i: usize| {
                if i == len_jobs {
                    return 0;
                }

                if dfs.cache[i] != -1 {
                    return dfs.cache[i];
                }

                // not include
                let mut res = (dfs.f)(dfs, i + 1);

                // include
                let mut j = i + 1;
                loop {
                    if j == len_jobs {
                        break;
                    }

                    if jobs[j].0 >= jobs[i].1 {
                        break;
                    }

                    j += 1;
                }

                res = std::cmp::max(res, jobs[i].2 + (dfs.f)(dfs, j));

                dfs.cache[i] = res;

                res
            },
        };

        (dfs.f)(&mut dfs, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_scheduling_1() {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];

        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 120)
    }

    #[test]
    fn test_job_scheduling_2() {
        let start_time = vec![1, 2, 3, 4, 6];
        let end_time = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];

        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 150)
    }

    #[test]
    fn test_job_scheduling_3() {
        let start_time = vec![1, 1, 1];
        let end_time = vec![2, 3, 4];
        let profit = vec![5, 6, 4];

        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 6)
    }

    #[test]
    fn test_job_scheduling_4() {
        let start_time = vec![6, 15, 7, 11, 1, 3, 16, 2];
        let end_time = vec![19, 18, 19, 16, 10, 8, 19, 8];
        let profit = vec![2, 9, 1, 19, 5, 7, 3, 19];

        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), 41)
    }
}
