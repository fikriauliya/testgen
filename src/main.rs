use testgen::problemspec::{generator::Generator, spec::*};

struct ProblemSpec {
    t: usize,
    k: usize,
    n: Vec<usize>,
    s: Vec<String>,
    hashed: Vec<usize>,
}
impl ProblemSpec {
    fn input_format(self) -> IOFormat {
        vec![
            IOElement::Line(vec![LineElement::Scalar(Scalar::Int(self.t))]),
            IOElement::Line(vec![LineElement::Scalar(Scalar::Int(self.k))]),
            IOElement::EmptyLine,
            IOElement::RawLine("Hello".to_string()),
            IOElement::RawLinesBounded(self.s.clone(), self.k),
            IOElement::LinesBounded(
                vec![
                    self.n.into_iter().map(Scalar::Int).collect(),
                    self.hashed.into_iter().map(Scalar::Int).collect(),
                ],
                self.t,
            ),
        ]
    }
}
fn main() {
    let spec = ProblemSpec {
        t: 2,
        k: 2,
        n: vec![1, 2, 3],
        hashed: vec![4, 5, 6],
        s: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    let output = spec.input_format().generate();
    println!("{}", output);
}
