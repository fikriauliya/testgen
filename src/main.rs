use testgen::cli::run;
use testgen::problemspec::spec::*;
use testgen::testspec::spec::*;

struct Spec {
    a: Vec<i64>,
    b: Vec<i64>,
    sum: Vec<i64>,
}
impl ProblemSpec for Spec {
    fn input_format(&self) -> IOFormat {
        vec![IOElement::LinesUnbounded(vec![
            self.a.iter().map(|&x| Scalar::Int(x)).collect(),
            self.b.iter().map(|&x| Scalar::Int(x)).collect(),
        ])]
    }

    fn output_format(&self) -> IOFormat {
        vec![IOElement::LinesUnbounded(vec![self
            .sum
            .iter()
            .map(|&x| Scalar::Int(x))
            .collect()])]
    }

    fn constraints(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if !self.a.iter().all(|it| *it >= 0) {
            errors.push("a >= 0".to_string());
        }
        if !self.b.iter().all(|it| *it >= 0) {
            errors.push("a >= 0".to_string());
        }
        if errors.is_empty() {
            return Ok(());
        } else {
            return Err(errors);
        }
    }
}
impl TestSpec<Spec> for Spec {
    fn test_cases() -> Vec<Spec> {
        vec![
            Spec {
                a: vec![1, 2, 3],
                b: vec![1, 2, 3],
                sum: vec![],
            },
            Spec {
                a: vec![4, 5, 6],
                b: vec![4, 5, 6],
                sum: vec![],
            },
        ]
    }
}

fn main() {
    run::<Spec>();
}
