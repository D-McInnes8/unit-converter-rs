use std::error::Error;
use std::fmt;

#[derive(Debug)]
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

impl Default for ConversionError {
    fn default() -> Self {
        ConversionError {
            source: None,
            message: None,
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.source {
            return write!(f, "{}", source.to_string());
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
