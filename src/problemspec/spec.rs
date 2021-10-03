pub type Size = usize;

pub type IOFormat = Vec<IOElement>;

#[derive(Debug, Clone)]
pub enum IOElement {
    Line(Vec<LineElement>),
    RawLine(String),
    EmptyLine,
    RawLinesBounded(Vec<String>, Size),
    RawLinesUnbounded(Vec<String>),
    LinesBounded(Vec<Vec<Scalar>>, Size),
    LinesUnbounded(Vec<Vec<Scalar>>),
    // Grid(Vec<Vec<Scalar>>, Size, Size),
}

#[derive(Debug, Clone)]
pub enum Scalar {
    Int(usize),
    Float(f64),
}

#[derive(Debug, Clone)]
pub enum LineElement {
    Scalar(Scalar),
    BoundedVec(Vec<Scalar>, Size),
    UnboundedVec(Vec<Scalar>),
}
