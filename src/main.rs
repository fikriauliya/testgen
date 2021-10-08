use testgen::cli::run;
use testgen::problemspec::spec::*;
use testgen::testspec::spec::*;
use testgen::{CONS, LINE, LS};

struct Spec {
    a: i64,
    b: i64,
    // sum: i64,
}
impl ProblemSpec for Spec {
    fn input_format(&self) -> IOFormat {
        vec![LINE!(LS!(self.a), LS!(self.b))]
    }

    // fn output_format(&self) -> IOFormat {
    //     vec![IOElement::Line(vec![LineElement::Scalar(Scalar::Int(
    //         self.sum,
    //     ))])]
    // }

    fn multiple_test_case_config() -> Option<MultipleTestcaseConfig> {
        Some(MultipleTestcaseConfig {
            constraints: |t| CONS!(t < 10),
            output_prefix: Some("Case #".to_string()),
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
            result.push(Spec { a, b });
        }
        result
    }
}

fn main() {
    run::<Spec>();
}
