use testgen::problemspec::{generator::Generator, spec::*};

struct ProblemSpec {
    t: u64,
    k: usize,
    m: usize,
    n: Vec<u64>,
    s: Vec<String>,
    hashed: Vec<u64>,
}
impl ProblemSpec {
    fn input_format(&self) -> IOFormat {
        vec![
            IOElement::Line(vec![LineElement::Scalar(Scalar::UInt(self.t))]),
            IOElement::EmptyLine,
            IOElement::RawLine("Hello".to_string()),
            IOElement::RawLinesBounded(self.s.clone(), self.k),
            IOElement::LinesBounded(
                vec![
                    self.n.iter().map(|&x| Scalar::UInt(x)).collect(),
                    self.hashed.iter().map(|&x| Scalar::UInt(x)).collect(),
                ],
                self.m,
            ),
        ]
    }

    fn constaints(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if !(self.t >= 4) {
            errors.push("t >= 4".to_string());
        }
        if !(self.k >= 1) {
            errors.push("k >= 1".to_string());
        }
        if errors.is_empty() {
            return Ok(());
        } else {
            return Err(errors);
        }
    }
}
fn main() {
    let spec = ProblemSpec {
        t: 2,
        k: 2,
        m: 2,
        n: vec![1, 2, 3],
        hashed: vec![4, 5, 6],
        s: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    let output = spec.input_format().generate();
    println!("{}", output);

    if let Err(errors) = spec.constaints() {
        println!("{:?}", errors);
    }
}
