use core::fmt;
use std::error::Error;

use crate::error::ExpressionError;

#[derive(Debug, Default)]
pub struct ParseError {
    source: Option<Box<dyn Error + 'static>>,
    message: Option<String>,
    token: String,
}

impl ParseError {
    pub fn new(message: &str, token: &str, source: Option<impl Error + 'static>) -> ParseError {
        ParseError {
            source: source.map(|s| -> Box<dyn Error> { Box::new(s) }),
            message: Some(message.to_owned()),
            token: token.to_owned(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
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
        write!(
            f,
            "Error while parsing token '{}'. {}",
            self.token, error_message
        )
    }
}

impl From<ParseError> for ExpressionError {
    fn from(value: ParseError) -> Self {
        ExpressionError::new(&format!("{}", value))
    }
}
