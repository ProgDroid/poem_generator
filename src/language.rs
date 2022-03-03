use std::fmt;

#[repr(u8)]
#[derive(Debug)]
pub enum Language {
    None = 0,
    Pt = 1,
    En = 2,
}

impl Language {
    pub fn to_note_label(&self) -> &str {
        match self {
            Language::Pt => "pt",
            Language::En => "english",
            Language::None => "none",
        }
    }

    pub fn converse(&self) -> Language {
        match self {
            Language::Pt => Language::En,
            Language::En => Language::Pt,
            Language::None => Language::None,
        }
    }
}

impl From<u8> for Language {
    fn from(value: u8) -> Language {
        match value {
            1 => Language::Pt,
            2 => Language::En,
            _ => Language::None,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
