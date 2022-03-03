use std::fmt;

#[derive(Debug)]
pub struct Poem(pub String);

impl fmt::Display for Poem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n============\n\n{}\n\n============", self.0)
    }
}

impl Poem {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
