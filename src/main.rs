use testgen::cli::run_multi;
use testgen::problemspec::spec::*;
use testgen::testspec::random::Random;
use testgen::testspec::spec::*;
use testgen::{CONS, LINE, LS};

struct Spec {
    a: i64,
    b: i64,
    sum: Option<i64>,
}
impl ProblemSpec<Spec> for Spec {
    fn input_format(&self) -> IOFormat {
        vec![LINE!(LS!(self.a), LS!(self.b))]
    }

    fn output_format(&self) -> IOFormat {
        vec![LINE!(LS!(self.sum.unwrap()))]
    }

    fn multiple_test_case_config() -> Option<MultipleTestcaseConfig> {
        Some(MultipleTestcaseConfig {
            constraints: |t| CONS!(t < 10),
            output_prefix: Some("Case #{}: ".to_string()),
        })
    }

    fn constraints(&self) -> Result<(), ConstraintsError> {
        CONS!(self.a > 0, self.b > 0)
    }
}

impl MultitaskProblemSpec<Spec> for Spec {
    fn subtask_1() -> Option<SubtaskConfig<Spec>> {
        Some(SubtaskConfig {
            score: 20,
            constraints: |s| CONS!(s.a <= 10),
        })
    }

    fn subtask_2() -> Option<SubtaskConfig<Spec>> {
        Some(SubtaskConfig {
            score: 80,
            constraints: |s| CONS!(s.b <= 100),
        })
    }
}

impl SingletaskTestSpec<Spec> for Spec {
    fn test_cases(random: &mut Random) -> Vec<Spec> {
        let mut result = Vec::new();
        for _ in 0..9 {
            result.push(Spec {
                a: random.next_range(1, 10),
                b: random.next_range(1, 10),
                sum: None,
            });
        }
        result
    }

    fn sample_test_cases() -> Vec<Spec> {
        vec![
            Spec {
                a: 1,
                b: 1,
                sum: Some(2),
            },
            Spec {
                a: 1,
                b: 2,
                sum: Some(3),
            },
        ]
    }
}
impl MultitaskTestSpec<Spec> for Spec {
    fn test_cases_subtask_1(random: &mut Random) -> Option<Vec<Spec>> {
        let mut result = Vec::new();
        for _ in 0..9 {
            result.push(Spec {
                a: random.next_range(1, 10),
                b: random.next_range(1, 10),
                sum: None,
            });
        }
        Some(result)
    }
    fn test_cases_subtask_2(random: &mut Random) -> Option<Vec<Spec>> {
        let mut result = Vec::new();
        for _ in 0..9 {
            result.push(Spec {
                a: random.next_range(1, 100),
                b: random.next_range(1, 100),
                sum: None,
            });
        }
        Some(result)
    }

    fn sample_test_cases() -> Vec<Spec> {
        vec![
            Spec {
                a: 1,
                b: 1,
                sum: Some(2),
            },
            Spec {
                a: 1,
                b: 2,
                sum: Some(3),
            },
        ]
    }
}

fn main() {
    run_multi::<Spec>();
    // Uncomment this and comment the previous line to turn on single-task mode
    // run::<Spec>();
}
