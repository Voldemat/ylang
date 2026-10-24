use crate::shared::EOF;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EOF,
    UnknownLoadCommandType(u32),
}

impl From<EOF> for ParseError {
    fn from(_: EOF) -> Self {
        Self::EOF
    }
}
