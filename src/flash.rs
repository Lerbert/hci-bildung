use std::convert::TryInto;

use rocket::http::uri::Reference;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::serde::Serialize;

#[derive(Responder)]
pub enum FlashRedirect {
    Redirect(Redirect),
    Flash(Flash<Redirect>),
}

impl FlashRedirect {
    pub fn with_flash<U: TryInto<Reference<'static>>, K: Into<String>, M: Into<String>>(
        uri: U,
        kind: K,
        message: M,
    ) -> Self {
        Self::Flash(Flash::new(Redirect::to(uri), kind, message))
    }

    pub fn no_flash<U: TryInto<Reference<'static>>>(uri: U) -> Self {
        Self::Redirect(Redirect::to(uri))
    }
}

#[derive(Serialize)]
pub struct FlashContext {
    kind: String,
    message: String,
}

impl<'r> From<FlashMessage<'r>> for FlashContext {
    fn from(flash_message: FlashMessage<'r>) -> Self {
        FlashContext {
            kind: flash_message.kind().to_string(),
            message: flash_message.message().to_string(),
        }
    }
}
