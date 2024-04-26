use std::error::Error;
use std::fmt;

/*#[derive(Debug, Default)]
pub struct ParseError {
    source: Option<Box<dyn Error + 'static>>,
    message: Option<String>,
    pub token: String,
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

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.source {
            return write!(f, "{}", source);
        }

        let error_message = match &self.message {
            Some(err) => err,
            None => "An unknown error has occurred",
        };

        write!(f, "Error while parsing token '{}'. ", self.token)?;
        write!(f, "{}", error_message)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}*/

#[derive(Debug, Default)]
pub struct ExpressionError {
    message: String,
    source: Option<Box<dyn Error + 'static>>,
}

impl ExpressionError {
    pub fn new(message: &str) -> ExpressionError {
        ExpressionError {
            source: None,
            message: message.to_owned(),
        }
    }

    pub fn from_source(message: &str, source: impl Error + 'static) -> ExpressionError {
        ExpressionError {
            source: Some(Box::new(source)),
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source {
            Some(source) => write!(f, "{} ({})", self.message, source),
            None => write!(f, "{}", self.message),
        }
    }
}

impl Error for ExpressionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}
