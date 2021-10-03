pub trait ProblemSpec {
    fn input_format(&self) -> IOFormat;
    fn output_format(&self) -> IOFormat;
    fn constraints(&self) -> Result<(), Vec<String>>;
}

pub type Size = usize;

pub type IOFormat = Vec<IOElement>;

#[derive(Debug, Clone)]
pub enum IOElement {
    Line(Vec<LineElement>),
    RawLine(String),
    EmptyLine,
    LinesBounded(Vec<Vec<Scalar>>, Size),
    LinesUnbounded(Vec<Vec<Scalar>>),
    RawLinesBounded(Vec<String>, Size),
    RawLinesUnbounded(Vec<String>),
    Grid(Vec<Vec<Scalar>>, Size, Size),
}

#[derive(Debug, Clone)]
pub enum Scalar {
    UInt(u64),
    Float(f64),
    String(String),
    Char(char),
    Int(i64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum LineElement {
    Scalar(Scalar),
    BoundedVec(Vec<Scalar>, Size),
    UnboundedVec(Vec<Scalar>),
}
