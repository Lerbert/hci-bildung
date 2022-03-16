use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

use crate::login::UserTransport;

pub use super::logic::Id;
use super::logic::{Sheet, SheetMetadata};

#[derive(Debug, Deserialize, Serialize)]
pub struct SheetTransport {
    pub id: Option<Id>,
    pub title: String,
    pub tiptap: serde_json::Value,
}

impl From<Sheet> for SheetTransport {
    fn from(sheet: Sheet) -> Self {
        SheetTransport {
            id: Some(sheet.metadata.id),
            title: sheet.metadata.title,
            tiptap: sheet.tiptap,
        }
    }
}

impl From<Sheet> for SheetOverviewTransport {
    fn from(sheet: Sheet) -> Self {
        SheetOverviewTransport::from(sheet.metadata)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SheetOverviewTransport {
    pub id: String,
    pub title: String,
    pub owner: UserTransport,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
}

impl From<SheetMetadata> for SheetOverviewTransport {
    fn from(metadata: SheetMetadata) -> Self {
        SheetOverviewTransport {
            id: metadata.id.to_string(),
            title: metadata.title,
            owner: metadata.owner,
            created: metadata.created,
            changed: metadata.changed,
        }
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
