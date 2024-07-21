use std::fmt;

#[derive(Debug)]
pub(super) enum SessionType {
    Verification,
}

impl fmt::Display for SessionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
