#[derive(Debug)]
pub struct ParseSlugError<Meta = self::Meta> {
    input: Box<str>,
    meta: Meta,
}

impl ParseSlugError {
    pub fn new(input: &str) -> ParseSlugError<()> {
        ParseSlugError {
            input: input.to_owned().into_boxed_str(),
            meta: (),
        }
    }
}

impl ParseSlugError<()> {
    pub fn meta(self, meta: impl Into<Meta>) -> ParseSlugError {
        ParseSlugError {
            input: self.input,
            meta: meta.into(),
        }
    }

    pub fn illegal_character(self, character: char) -> ParseSlugError {
        self.meta(IllegalCharacter { character })
    }

    pub fn empty_input(self) -> ParseSlugError {
        self.meta(Meta::EmptyInput)
    }

    pub fn hanging_underscore(self) -> ParseSlugError {
        self.meta(Meta::HangingUnderscore)
    }

    pub fn only_underscores(self) -> ParseSlugError {
        self.meta(Meta::OnlyUnderscores)
    }
}

impl std::fmt::Display for ParseSlugError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("slug must not contain slashes")
    }
}

#[derive(Debug)]
pub enum Meta {
    EmptyInput,
    IllegalCharacter(IllegalCharacter),
    HangingUnderscore,
    OnlyUnderscores,
}

#[derive(Debug)]
pub struct IllegalCharacter {
    character: char,
}

impl From<IllegalCharacter> for Meta {
    fn from(value: IllegalCharacter) -> Self {
        Self::IllegalCharacter(value)
    }
}
