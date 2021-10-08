use super::spec::Scalar;
use std::convert::From;

impl From<i32> for Scalar {
    fn from(x: i32) -> Self {
        Scalar::Int(x as i64)
    }
}
impl From<i64> for Scalar {
    fn from(x: i64) -> Self {
        Scalar::Int(x)
    }
}
impl From<u32> for Scalar {
    fn from(x: u32) -> Self {
        Scalar::UInt(x as u64)
    }
}
impl From<u64> for Scalar {
    fn from(x: u64) -> Self {
        Scalar::UInt(x)
    }
}
impl From<f64> for Scalar {
    fn from(x: f64) -> Self {
        Scalar::Float(x)
    }
}
impl From<String> for Scalar {
    fn from(x: String) -> Self {
        Scalar::String(x)
    }
}
impl From<&str> for Scalar {
    fn from(x: &str) -> Self {
        Scalar::String(String::from(x))
    }
}
impl From<char> for Scalar {
    fn from(x: char) -> Self {
        Scalar::Char(x)
    }
}
impl From<bool> for Scalar {
    fn from(x: bool) -> Self {
        Scalar::Bool(x)
    }
}
