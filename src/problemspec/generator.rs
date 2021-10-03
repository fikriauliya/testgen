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
            IOElement::RawLinesBounded(lines, size) => {
                let mut result = String::new();
                for i in 0..*size {
                    result.push_str(lines[i].as_str());
                    if i != *size - 1 {
                        result.push('\n');
                    }
                }
                result
            }
            IOElement::RawLinesUnbounded(lines) => lines.join("\n"),
        }
    }
}

impl Generator for Scalar {
    fn generate(&self) -> String {
        match self {
            Scalar::Int(i) => i.to_string(),
            Scalar::Float(f) => f.to_string(),
        }
    }
}

impl Generator for LineElement {
    fn generate(&self) -> String {
        match self {
            LineElement::Scalar(s) => s.generate(),
            LineElement::BoundedVec(v, size) => {
                let mut result = String::new();
                for i in 0..*size {
                    result.push_str(&v[i].generate());
                    if i != *size - 1 {
                        result.push_str(" ");
                    }
                }
                result
            }
            LineElement::UnboundedVec(v) => v
                .iter()
                .map(|s| s.generate())
                .collect::<Vec<String>>()
                .join(" "),
        }
    }
}
