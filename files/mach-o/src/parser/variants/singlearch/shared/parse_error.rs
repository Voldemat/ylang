use gshared::eof::EOF;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EOF,
    ExpectedEmptyBuffer(usize),
    UnknownLoadCommandType(u32),
}

impl From<EOF> for ParseError {
    fn from(_: EOF) -> Self {
        Self::EOF
    }
}
