use super::spec::*;

pub trait Generator {
    fn generate(&self) -> String;
}

impl Generator for IOFormat {
    fn generate(&self) -> String {
        self.iter()
            .map(|e| e.generate())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Generator for IOElement {
    fn generate(&self) -> String {
        match self {
            IOElement::Line(line) => line
                .iter()
                .map(|element| element.generate())
                .collect::<Vec<String>>()
                .join(" "),
            IOElement::LinesBounded(lines, size) => {
                let mut result = String::new();
                for i in 0..*size {
                    for (pos, line) in lines.iter().enumerate() {
                        result.push_str(line[i].generate().as_str());
                        if pos != lines.len() - 1 {
                            result.push_str(" ");
                        }
                    }
                    if i != *size - 1 {
                        result.push_str("\n");
                    }
                }
                result
            }
            IOElement::LinesUnbounded(lines) => {
                let mut result = String::new();
                for i in 0..lines[0].len() {
                    for (pos, line) in lines.iter().enumerate() {
                        result.push_str(line[i].generate().as_str());
                        if pos != lines.len() - 1 {
                            result.push_str(" ");
                        }
                    }
                    if i != lines[0].len() - 1 {
                        result.push('\n');
                    }
                }
                result
            }
            IOElement::RawLine(line) => line.to_string(),
            IOElement::EmptyLine => String::new(),
            IOElement::RawLinesBounded(lines, size) => lines
                .into_iter()
                .take(*size)
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            IOElement::RawLinesUnbounded(lines) => lines.join("\n"),
            IOElement::Grid(grid, height, width) => {
                let mut result = String::new();
                for i in 0..*height {
                    for j in 0..*width {
                        result.push_str(grid[i][j].generate().as_str());
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
                result
            }
        }
    }
}

impl Generator for Scalar {
    fn generate(&self) -> String {
        match self {
            Scalar::UInt(u) => u.to_string(),
            Scalar::Int(i) => i.to_string(),
            Scalar::Float(f) => f.to_string(),
            Scalar::String(s) => s.to_string(),
            Scalar::Char(c) => c.to_string(),
            Scalar::Bool(b) => b.to_string(),
        }
    }
}

impl Generator for LineElement {
    fn generate(&self) -> String {
        match self {
            LineElement::Scalar(s) => s.generate(),
            LineElement::BoundedVec(v, size) => v
                .iter()
                .map(|e| e.generate())
                .take(*size)
                .collect::<Vec<String>>()
                .join(" "),
            LineElement::UnboundedVec(v) => v
                .iter()
                .map(|s| s.generate())
                .collect::<Vec<String>>()
                .join(" "),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_line() {
        let line = LINE!(
            LS!(1),
            LineElement::BoundedVec(vec![Scalar::Int(2), Scalar::Int(3)], 2),
            LV![4, 5],
            LS!(6.5),
        );
        let result = line.generate();
        assert_eq!(result, "1 2 3 4 5 6.5");
    }

    #[test]
    fn test_generate_lines_bounded() {
        let a = V![1, 2, 3];
        let b = V![4, 5, 6];
        let lines = vec![a, b];
        let result = IOElement::LinesBounded(lines.clone(), 3).generate();
        assert_eq!(result, "1 4\n2 5\n3 6");

        let result = IOElement::LinesBounded(lines.clone(), 1).generate();
        assert_eq!(result, "1 4");
    }

    #[test]
    fn test_generate_lines_unbounded() {
        //TODO: support vector
        let a = V![1, 2, 3];
        let b = V![4, 5, 6];
        let result = LINES!(a, b).generate();
        assert_eq!(result, "1 4\n2 5\n3 6");
    }

    #[test]
    fn test_generate_raw_line() {
        let line = RAW_LINE!("Hello World");
        let result = line.generate();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_generate_empty_line() {
        let line = EMPTY_LINE!();
        let result = line.generate();
        assert_eq!(result, "");
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
        assert_eq!(result, "Hello World\nHello World\nHello World");

        let line = IOElement::RawLinesBounded(lines.clone(), 2);
        let result = line.generate();
        assert_eq!(result, "Hello World\nHello World");
    }

    #[test]
    fn test_generate_raw_lines_unbounded() {
        let line = RAW_LINES!("Hello", "World");
        let result = line.generate();
        assert_eq!(result, "Hello\nWorld");
    }

    #[test]
    fn test_generate_grid() {
        //TODO: make macro
        let grid = vec![V![1, 2, 3], V![4, 5, 6]];
        let result = IOElement::Grid(grid, 2, 3).generate();
        assert_eq!(result, "1 2 3\n4 5 6");

        let grid = vec![V!['a', 'b', 'c'], V!['d', 'e', 'f']];
        let result = IOElement::Grid(grid, 2, 3).generate();
        assert_eq!(result, "abc\ndef");

        let grid = vec![V!["Hello", "World"], V!["Hi", "All"]];
        let result = IOElement::Grid(grid, 2, 2).generate();
        assert_eq!(result, "Hello World\nHi All");
    }

    #[test]
    fn test_generate_scalar() {
        let scalar = Scalar::UInt(42);
        assert_eq!(scalar.generate(), "42");

        let scalar = Scalar::Int(-42);
        assert_eq!(scalar.generate(), "-42");

        let scalar = Scalar::Float(3.14);
        assert_eq!(scalar.generate(), "3.14");

        let scalar = Scalar::String("Hello World".to_string());
        assert_eq!(scalar.generate(), "Hello World");

        let scalar = Scalar::Char('a');
        assert_eq!(scalar.generate(), "a");

        let scalar = Scalar::Bool(true);
        assert_eq!(scalar.generate(), "true");
    }

    #[test]
    fn test_generate_line_element() {
        let element = LS!(1);
        assert_eq!(element.generate(), "1");
        let element = LineElement::BoundedVec(V![1, 2], 2);
        assert_eq!(element.generate(), "1 2");
        let element = LineElement::BoundedVec(V![1, 2], 1);
        assert_eq!(element.generate(), "1");
        let element = LV![1, 2];
        assert_eq!(element.generate(), "1 2");
    }
}
