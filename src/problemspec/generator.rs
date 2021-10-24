use super::spec::*;

pub trait Generator {
    fn generate(&self) -> Option<String>;
}

impl Generator for IOFormat {
    fn generate(&self) -> Option<String> {
        Some(
            self.iter()
                .map(|e| e.generate())
                .filter(|e| e.is_some())
                .map(|e| e.unwrap())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl Generator for IOElement {
    fn generate(&self) -> Option<String> {
        match self {
            IOElement::Line(line) => Some(
                line.iter()
                    .map(|element| element.generate().unwrap())
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
            IOElement::LinesBounded(lines, size) => {
                let mut result = String::new();
                for i in 0..*size {
                    for (pos, line) in lines.iter().enumerate() {
                        result.push_str(line[i].generate().unwrap().as_str());
                        if pos != lines.len() - 1 {
                            result.push_str(" ");
                        }
                    }
                    if i != *size - 1 {
                        result.push_str("\n");
                    }
                }
                if result.is_empty() {
                    None
                } else {
                    Some(result)
                }
            }
            IOElement::LinesUnbounded(lines) => {
                let mut result = String::new();
                for i in 0..lines[0].len() {
                    for (pos, line) in lines.iter().enumerate() {
                        result.push_str(line[i].generate().unwrap().as_str());
                        if pos != lines.len() - 1 {
                            result.push_str(" ");
                        }
                    }
                    if i != lines[0].len() - 1 {
                        result.push('\n');
                    }
                }
                if result.is_empty() {
                    None
                } else {
                    Some(result)
                }
            }
            IOElement::RawLine(line) => Some(line.to_string()),
            IOElement::EmptyLine => Some("".to_string()),
            IOElement::RawLinesBounded(lines, size) => Some(
                lines
                    .into_iter()
                    .take(*size)
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
            IOElement::RawLinesUnbounded(lines) => Some(lines.join("\n")),
            IOElement::Grid(grid, height, width) => {
                let mut result = String::new();
                for i in 0..*height {
                    for j in 0..*width {
                        result.push_str(grid[i][j].generate().unwrap().as_str());
                        if j != *width - 1 {
                            match grid[i][j] {
                                Scalar::Char(_) => (),
                                _ => result.push(' '),
                            }
                        }
                    }
                    if i != *height - 1 {
                        result.push('\n');
                    }
                }
                Some(result)
            }
        }
    }
}

impl Generator for Scalar {
    fn generate(&self) -> Option<String> {
        Some(match self {
            Scalar::UInt(u) => u.to_string(),
            Scalar::Int(i) => i.to_string(),
            Scalar::Float(f) => f.to_string(),
            Scalar::String(s) => s.to_string(),
            Scalar::Char(c) => c.to_string(),
            Scalar::Bool(b) => b.to_string(),
        })
    }
}

impl Generator for LineElement {
    fn generate(&self) -> Option<String> {
        match self {
            LineElement::Scalar(s) => s.generate(),
            LineElement::BoundedVec(v, size) => Some(
                v.iter()
                    .map(|e| e.generate().unwrap())
                    .take(*size)
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
            LineElement::UnboundedVec(v) => Some(
                v.iter()
                    .map(|s| s.generate().unwrap())
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{EMPTY_LINE, LINE, LINES, LS, LV, RAW_LINE, RAW_LINES, V};

    use super::*;

    #[test]
    fn test_generate_io_format() {
        let ios = vec![LINE!(LS!(1)), LINES!(V![vec![1, 2]])];
        let result = ios.generate();
        assert_eq!(result, Some("1\n1\n2".to_string()));

        let ios = vec![
            LINE!(LS!(1)),
            LINES!(V![Vec::new() as Vec<u32>]),
            LINES!(V![Vec::new() as Vec<u32>]),
        ];
        let result = ios.generate();
        assert_eq!(result, Some("1".to_string()));

        let ios = vec![
            LINE!(LS!(1)),
            LINES!(V![Vec::new() as Vec<u32>]),
            LINES!(V![Vec::new() as Vec<u32>]),
            LINE!(LS!(2)),
        ];
        let result = ios.generate();
        assert_eq!(result, Some("1\n2".to_string()));

        let ios = vec![LINE!(LS!(1)), EMPTY_LINE!(), LINE!(LS!(2))];
        dbg!(&ios);
        dbg!(ios.generate());
        let result = ios.generate();
        assert_eq!(result, Some("1\n\n2".to_string()));
    }

    #[test]
    fn test_generate_line() {
        let line = LINE!(
            LS!(1),
            LineElement::BoundedVec(vec![Scalar::Int(2), Scalar::Int(3)], 2),
            LV![vec![4, 5]],
            LS!(6.5),
        );
        let result = line.generate();
        assert_eq!(result, Some("1 2 3 4 5 6.5".to_string()));
    }

    #[test]
    fn test_generate_lines_bounded() {
        let a = V![vec![1, 2, 3]];
        let b = V![vec![4, 5, 6]];
        let lines = vec![a, b];
        let result = IOElement::LinesBounded(lines.clone(), 3).generate();
        assert_eq!(result, Some("1 4\n2 5\n3 6".to_string()));

        let result = IOElement::LinesBounded(lines.clone(), 1).generate();
        assert_eq!(result, Some("1 4".to_string()));

        let result = IOElement::LinesBounded(lines.clone(), 0).generate();
        assert_eq!(result, None);

        let a = V![Vec::new() as Vec<i32>];
        let lines = vec![a];
        let result = IOElement::LinesBounded(lines.clone(), 0).generate();
        assert_eq!(result, None);

        //TODO: Handle n < a.len(), b.len()
    }

    #[test]
    fn test_generate_lines_unbounded() {
        let a = V![vec![1, 2, 3]];
        let b = V![vec![4, 5, 6]];
        let result = LINES!(a, b).generate();
        assert_eq!(result, Some("1 4\n2 5\n3 6".to_string()));

        let a = V![Vec::new() as Vec<u32>];
        let result = LINES!(a).generate();
        assert_eq!(result, None);
    }

    #[test]
    fn test_generate_raw_line() {
        let line = RAW_LINE!("Hello World");
        let result = line.generate();
        assert_eq!(result, Some("Hello World".to_string()));
    }

    #[test]
    fn test_generate_empty_line() {
        let line = EMPTY_LINE!();
        let result = line.generate();
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn test_generate_raw_lines_bounded() {
        let lines = vec![
            "Hello World".to_string(),
            "Hello World".to_string(),
            "Hello World".to_string(),
        ];
        let line = IOElement::RawLinesBounded(lines.clone(), 3);
        let result = line.generate();
        assert_eq!(
            result,
            Some("Hello World\nHello World\nHello World".to_string())
        );

        let line = IOElement::RawLinesBounded(lines.clone(), 2);
        let result = line.generate();
        assert_eq!(result, Some("Hello World\nHello World".to_string()));
    }

    #[test]
    fn test_generate_raw_lines_unbounded() {
        let line = RAW_LINES!("Hello", "World");
        let result = line.generate();
        assert_eq!(result, Some("Hello\nWorld".to_string()));
    }

    #[test]
    fn test_generate_grid() {
        //TODO: make macro
        let grid = vec![V![vec![1, 2, 3]], V![vec![4, 5, 6]]];
        let result = IOElement::Grid(grid, 2, 3).generate();
        assert_eq!(result, Some("1 2 3\n4 5 6".to_string()));

        let grid = vec![V![vec!['a', 'b', 'c']], V![vec!['d', 'e', 'f']]];
        let result = IOElement::Grid(grid, 2, 3).generate();
        assert_eq!(result, Some("abc\ndef".to_string()));

        let grid = vec![V![vec!["Hello", "World"]], V![vec!["Hi", "All"]]];
        let result = IOElement::Grid(grid, 2, 2).generate();
        assert_eq!(result, Some("Hello World\nHi All".to_string()));
    }

    #[test]
    fn test_generate_scalar() {
        let scalar = Scalar::UInt(42);
        assert_eq!(scalar.generate(), Some("42".to_string()));

        let scalar = Scalar::Int(-42);
        assert_eq!(scalar.generate(), Some("-42".to_string()));

        let scalar = Scalar::Float(3.14);
        assert_eq!(scalar.generate(), Some("3.14".to_string()));

        let scalar = Scalar::String("Hello World".to_string());
        assert_eq!(scalar.generate(), Some("Hello World".to_string()));

        let scalar = Scalar::Char('a');
        assert_eq!(scalar.generate(), Some("a".to_string()));

        let scalar = Scalar::Bool(true);
        assert_eq!(scalar.generate(), Some("true".to_string()));
    }

    #[test]
    fn test_generate_line_element() {
        let element = LS!(1);
        assert_eq!(element.generate(), Some("1".to_string()));
        let element = LineElement::BoundedVec(V![vec![1, 2]], 2);
        assert_eq!(element.generate(), Some("1 2".to_string()));
        let element = LineElement::BoundedVec(V![vec![1, 2]], 1);
        assert_eq!(element.generate(), Some("1".to_string()));
        let element = LV![vec![1, 2]];
        assert_eq!(element.generate(), Some("1 2".to_string()));
    }
}
