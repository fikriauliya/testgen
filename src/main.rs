use testgen::cli::run;
use testgen::problemspec::spec::*;
use testgen::testspec::spec::*;

struct Spec {
    a: i64,
    b: i64,
    // sum: i64,
}
impl ProblemSpec for Spec {
    fn input_format(&self) -> IOFormat {
        vec![IOElement::Line(vec![
            LineElement::Scalar(Scalar::Int(self.a)),
            LineElement::Scalar(Scalar::Int(self.b)),
        ])]
    }

    // fn output_format(&self) -> IOFormat {
    //     vec![IOElement::Line(vec![LineElement::Scalar(Scalar::Int(
    //         self.sum,
    //     ))])]
    // }

    fn multiple_test_case_config() -> Option<MultipleTestcaseConfig> {
        Some(MultipleTestcaseConfig {
            constraints: |t| {
                let mut errors = Vec::new();
                if !(t < 10) {
                    errors.push("t < 10".to_string());
                }
                if errors.is_empty() {
                    return Ok(());
                } else {
                    return Err(ConstraintsError { messages: errors });
                }
            },
            output_prefix: Some("Case #".to_string()),
        })
    }

    fn constraints(&self) -> Result<(), ConstraintsError> {
        let mut errors = Vec::new();
        if !(self.a > 0) {
            errors.push("a > 0".to_string());
        }
        if !(self.b > 0) {
            errors.push("b > 0".to_string());
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(ConstraintsError { messages: errors })
        }
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
