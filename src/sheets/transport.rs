use std::fmt::{self, Display};

use rocket::serde::{Deserialize, Serialize};

use crate::validation::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct SheetTransport {
    pub title: String,
    pub content: serde_json::Value,
}

#[derive(Debug)]
pub enum SheetTransportValidationError {
    TitleEmpty,
}

impl Display for SheetTransportValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TitleEmpty => write!(f, "Title cannot be empty"),
        }
    }
}

impl Validate for SheetTransport {
    type ValidationError = SheetTransportValidationError;

    fn validate(&self) -> Result<(), Self::ValidationError> {
        if self.title.is_empty() {
            return Err(Self::ValidationError::TitleEmpty);
        }
        Ok(())
    }
}

#[derive(Debug, FromForm)]
pub struct NewSheetForm {
    #[field(validate = neq(""))]
    pub title: String,
}

#[derive(Debug, FromForm)]
pub struct ImportSheetForm {
    pub file: String,
}
