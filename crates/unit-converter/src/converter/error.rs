use std::error::Error;
use std::fmt;

use expr::error::ExpressionError;

use crate::graph::GraphOperationError;

#[derive(Debug, Default)]
pub struct ConversionError {
    source: Option<Box<dyn Error>>,
    message: Option<String>,
}

impl ConversionError {
    pub fn new(message: &str) -> ConversionError {
        ConversionError {
            source: None,
            message: Some(message.to_string()),
        }
    }
}

impl fmt::Display for ConversionError {
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

impl Error for ConversionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

impl From<std::io::Error> for ConversionError {
    fn from(value: std::io::Error) -> Self {
        ConversionError::new(&value.to_string())
    }
}

impl From<toml::de::Error> for ConversionError {
    fn from(value: toml::de::Error) -> Self {
        ConversionError::new(value.message())
    }
}

// TODO: Fix up this from trait so that it contains a proper error message
impl From<ExpressionError> for ConversionError {
    fn from(_value: ExpressionError) -> Self {
        ConversionError::new("")
    }
}

// TODO: Fix up this from trait so that it contains a proper error message
impl From<GraphOperationError> for ConversionError {
    fn from(_value: GraphOperationError) -> Self {
        ConversionError::new("")
    }
}
