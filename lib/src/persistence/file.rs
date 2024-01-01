use std::ffi::OsString;

use crate::parser::parse_conversion;
use crate::{ConversionStore, UnitConversion};

pub struct FileConversionStore {
    file_path: OsString,
}

impl ConversionStore for FileConversionStore {
    fn get_default_conversions(&self) -> Result<Vec<UnitConversion>, ()> {
        let mut result = Vec::<UnitConversion>::new();

        match std::fs::read_to_string(&self.file_path) {
            Ok(file) => {
                for line in file.lines() {
                    _ = parse_conversion(line).map(|conversion| {
                        result.push(conversion);
                    });
                }
            }
            Err(err) => return Err(()),
        };

        Ok(result)
    }
}
