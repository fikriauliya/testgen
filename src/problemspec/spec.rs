use thiserror::Error;

#[derive(Debug, Error)]
#[error("Expected: {messages:?}")]
pub struct ConstraintsError {
    pub messages: Vec<String>,
}

pub trait ProblemSpec<T> {
    fn input_format(&self) -> IOFormat;
    // TODO: check output format
    fn output_format(&self) -> IOFormat;
    fn constraints(&self) -> Result<(), ConstraintsError>;
    fn multiple_test_case_config() -> Option<MultipleTestcaseConfig> {
        None
    }
}

pub trait MultitaskProblemSpec<T> {
    fn subtask_1() -> Option<SubtaskConfig<T>> {
        None
    }
    fn subtask_2() -> Option<SubtaskConfig<T>> {
        None
    }
    fn subtask_3() -> Option<SubtaskConfig<T>> {
        None
    }
}

pub type Size = usize;

pub struct MultipleTestcaseConfig {
    pub constraints: fn(usize) -> Result<(), ConstraintsError>,
    pub output_prefix: Option<String>,
}

pub struct SubtaskConfig<T> {
    pub score: u8,
    pub constraints: fn(&T) -> Result<(), ConstraintsError>,
}

pub type IOFormat = Vec<IOElement>;

#[derive(Debug, Clone, PartialEq)]
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

#[macro_export]
macro_rules! LINE {
    ($($x : expr), + $(,) ?) => {
        {
            let mut contents = Vec::new();
            $(
                contents.push($x);
            )*
            IOElement::Line(contents)
        }
    };
}

#[macro_export]
macro_rules! RAW_LINE {
    ($x:expr) => {
        IOElement::RawLine($x.to_string())
    };
}

#[macro_export]
macro_rules! EMPTY_LINE {
    () => {
        IOElement::EmptyLine
    };
}

#[macro_export]
macro_rules! LINES {
    ($($x:expr), + $(,) ?) => {
        IOElement::LinesUnbounded(vec![$($x), +])
    };
}

#[macro_export]
macro_rules! RAW_LINES {
    ($($x:expr), + $(,) ?) => {
        IOElement::RawLinesUnbounded(vec![$(String::from($x)), +])
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scalar {
    UInt(u64),
    Float(f64),
    String(String),
    Char(char),
    Int(i64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineElement {
    Scalar(Scalar),
    BoundedVec(Vec<Scalar>, Size),
    UnboundedVec(Vec<Scalar>),
}

#[macro_export]
macro_rules! LS {
    ($x:expr) => {{
        let x = $x;
        let result: Scalar = x.into();
        LineElement::Scalar(result)
    }};
}

#[macro_export]
macro_rules! LV {
    ($($x:expr), + $(,) ?) => {{
        LineElement::UnboundedVec(V![$($x), +])
    }};
}

#[macro_export]
macro_rules! V {
    ($($x : expr), + $(,) ?) => {{
        let res:Vec<Scalar> = vec![$($x), +].into_iter().map(|x| x.into()).collect();
        res
    }}
}

#[macro_export]
macro_rules! CONS {
    ($($x : expr), + $(,) ?) => {
        {
            let mut errors = Vec::new();
            $(
                if !($x) {
                    errors.push(String::from(stringify!($x)));
                }
            )*
            if errors.is_empty() {
                Ok(())
            } else {
                Err(ConstraintsError { messages: errors })
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_macro() {
        assert_eq!(
            LINE!(LineElement::Scalar(Scalar::UInt(1))),
            IOElement::Line(vec![LineElement::Scalar(Scalar::UInt(1)),])
        );

        assert_eq!(
            LINE!(
                LineElement::Scalar(Scalar::UInt(1)),
                LineElement::Scalar(Scalar::Int(2))
            ),
            IOElement::Line(vec![
                LineElement::Scalar(Scalar::UInt(1)),
                LineElement::Scalar(Scalar::Int(2)),
            ])
        );
    }

    #[test]
    fn test_raw_line_macro() {
        assert_eq!(RAW_LINE!("hello"), IOElement::RawLine("hello".to_string()));
    }

    #[test]
    fn test_generate_empty_line_macro() {
        assert_eq!(EMPTY_LINE!(), IOElement::EmptyLine);
    }

    #[test]
    fn test_v_macro() {
        assert_eq!(V![1], vec![Scalar::Int(1)]);
        assert_eq!(V![1.5, 2.3], vec![Scalar::Float(1.5), Scalar::Float(2.3)]);
        assert_eq!(V!['H', 'E'], vec![Scalar::Char('H'), Scalar::Char('E')]);
        assert_eq!(
            V!["hello", "world"],
            vec![
                Scalar::String("hello".to_string()),
                Scalar::String("world".to_string())
            ]
        );
    }

    #[test]
    fn test_ls_macro() {
        assert_eq!(LS!(1), LineElement::Scalar(Scalar::Int(1)));
        assert_eq!(LS!(1.5), LineElement::Scalar(Scalar::Float(1.5)));
        assert_eq!(LS!('H'), LineElement::Scalar(Scalar::Char('H')));
    }

    #[test]
    fn test_lv_macro() {
        assert_eq!(LV![1], LineElement::UnboundedVec(vec![Scalar::Int(1)]));
        assert_eq!(
            LV![1.5, 2.3],
            LineElement::UnboundedVec(vec![Scalar::Float(1.5), Scalar::Float(2.3)])
        );
    }

    #[test]
    fn test_lines_macro() {
        assert_eq!(
            LINES!(V![1, 2, 3], V![4, 5, 6]),
            IOElement::LinesUnbounded(vec![
                vec![Scalar::Int(1), Scalar::Int(2), Scalar::Int(3)],
                vec![Scalar::Int(4), Scalar::Int(5), Scalar::Int(6)],
            ])
        );
    }

    #[test]
    fn test_raw_lines_macro() {
        assert_eq!(
            RAW_LINES!("hello", "world"),
            IOElement::RawLinesUnbounded(vec!["hello".to_string(), "world".to_string()])
        )
    }
}
