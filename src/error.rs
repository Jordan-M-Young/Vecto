use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct MismatchError;

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyVectorError;

#[derive(Debug, Clone, PartialEq)]
pub struct BadTypeError;

#[derive(Debug, Clone, PartialEq)]
pub struct NonUniformError;

#[derive(Debug, Clone, PartialEq)]
pub struct NotImplementedError;

#[derive(Debug, Clone, PartialEq)]
pub enum CustomErrors {
    EmptyVector(EmptyVectorError),
    Mismatch(MismatchError),
    NonUniform(NonUniformError),
    BadType(BadTypeError),
    NotImplemented(NotImplementedError),
}

impl fmt::Display for CustomErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix dimensions are mismatched.")
    }
}
