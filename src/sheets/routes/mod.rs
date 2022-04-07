use rocket::http::Status;

use crate::flash::FlashRedirect;
use crate::login;
use crate::status::ToStatus;

use super::logic;
use super::transport;

pub mod sheet;
pub mod sheet_tree;
pub mod solution;

pub const MOUNT: &str = "/sheets";

impl ToStatus for logic::Error {
    fn to_status(self) -> Status {
        match self {
            Self::NotFound(_) => {
                debug!("{}", self);
                Status::NotFound
            }
            Self::Forbidden(_) => {
                info!("{}", self);
                Status::Forbidden
            }
            _ => {
                error!("{}", self);
                Status::InternalServerError
            }
        }
    }
}

impl ToStatus for transport::SheetTransportValidationError {
    fn to_status(self) -> Status {
        error!("Sheet validation failed: {}", self);
        Status::BadRequest
    }
}

pub fn sheets_uri(uri: rocket::http::uri::Origin) -> String {
    format!("{}{}", MOUNT, uri)
}

fn redirect_to_login() -> FlashRedirect {
    FlashRedirect::with_flash(
        uri!(login::routes::login),
        "danger",
        "Anmeldung erforderlich",
    )
}
