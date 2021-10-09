use crate::problemspec::spec::{MultitaskProblemSpec, ProblemSpec};

use super::random::Random;

pub trait SingletaskTestSpec<T>
where
    T: ProblemSpec<T>,
{
    fn sample_test_cases() -> Vec<T>;
    fn test_cases(random: &mut Random) -> Vec<T>;
}

pub trait MultitaskTestSpec<T>
where
    T: ProblemSpec<T> + MultitaskProblemSpec<T>,
{
    fn sample_test_cases() -> Vec<T>;
    fn test_cases_subtask_1(_random: &mut Random) -> Option<Vec<T>> {
        None
    }
    fn test_cases_subtask_2(_random: &mut Random) -> Option<Vec<T>> {
        None
    }
    fn test_cases_subtask_3(_random: &mut Random) -> Option<Vec<T>> {
        None
    }
}
