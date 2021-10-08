use testgen::cli::run;
use testgen::problemspec::spec::*;
use testgen::testspec::spec::*;
use testgen::{CONS, LINE, LS};

struct Spec {
    a: i64,
    b: i64,
    sum: Option<i64>,
}
impl ProblemSpec for Spec {
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
impl TestSpec<Spec> for Spec {
    fn test_cases(random: &mut Random) -> Vec<Spec> {
        let mut result = Vec::new();
        for _ in 0..9 {
            let a = random.next_range(1, 10);
            let b = random.next_range(1, 10);
            result.push(Spec { a, b, sum: None });
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

fn main() {
    run::<Spec>();
}
