pub type Size = usize;

pub type IOFormat = Vec<IOElement>;

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

pub enum Scalar {
    Int(usize),
    Float(f64),
}

pub enum LineElement {
    Scalar(Scalar),
    BoundedVec(Vec<Scalar>, Size),
    UnboundedVec(Vec<Scalar>),
}
