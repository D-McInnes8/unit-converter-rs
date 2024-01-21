use std::error::Error;
use std::fmt;

#[derive(Debug, Default)]
pub struct ParseError {
    source: Option<Box<dyn Error + 'static>>,
    message: Option<String>,
    token: Option<String>,
}

impl ParseError {
    pub fn new(message: &str, token: &str, source: Option<impl Error + 'static>) -> ParseError {
        ParseError {
            source: source.map(|s| -> Box<dyn Error> { Box::new(s) }),
            message: Some(message.to_owned()),
            token: Some(token.to_owned()),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.source {
            return write!(f, "{}", source);
        }

        let error_message = match &self.message {
            Some(err) => err,
            None => "An unknown error has occurred",
        };
        write!(f, "{}", error_message)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}
