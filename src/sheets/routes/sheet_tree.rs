use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use crate::flash::{FlashContext, FlashRedirect};
use crate::login::guards::{AuthenticatedUser, Teacher};
use crate::login::transport::UserTransport;
use crate::status::ToStatus;
use crate::Db;

use super::logic::sheet::SheetMetadata;
use super::{handle_insufficient_permissions, logic};

#[derive(Serialize)]
struct SheetManagementContext<'a> {
    flash: Option<FlashContext>,
    sheets: Vec<SheetMetadata>,
    user: &'a UserTransport,
}

#[get("/assignments")]
pub async fn assignment_overview(
    db: Db,
    teacher: Teacher<'_>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Status> {
    let user = teacher.into_inner();
    logic::sheet::get_all_sheets(&db, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|sheets| {
            Template::render(
                "management/sheet/my_sheets",
                &SheetManagementContext {
                    flash: flash.map(|f| f.into()),
                    sheets,
                    user: &user.user_info,
                },
            )
        })
}

#[get("/assignments", rank = 2)]
pub fn login_assignment_overview(
    user: Option<&AuthenticatedUser>,
) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[get("/assignments/trash")]
pub async fn trashed_sheets(db: Db, teacher: Teacher<'_>) -> Result<Template, Status> {
    let user = teacher.into_inner();
    logic::sheet::get_trash(&db, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|sheets| {
            Template::render(
                "management/sheet/trash",
                &SheetManagementContext {
                    flash: None,
                    sheets,
                    user: &user.user_info,
                },
            )
        })
}

#[get("/assignments/trash", rank = 2)]
pub fn login_trashed_sheets(user: Option<&AuthenticatedUser>) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}

#[get("/assignments/recent")]
pub async fn recent_sheets(db: Db, teacher: Teacher<'_>) -> Result<Template, Status> {
    let user = teacher.into_inner();
    logic::sheet::get_recent(&db, user.user_info.id)
        .await
        .map_err(|e| e.to_status())
        .map(|sheets| {
            Template::render(
                "management/sheet/recent_sheets",
                &SheetManagementContext {
                    flash: None,
                    sheets,
                    user: &user.user_info,
                },
            )
        })
}

#[get("/assignments/recent", rank = 2)]
pub fn login_recent_sheets(user: Option<&AuthenticatedUser>) -> Result<FlashRedirect, Status> {
    handle_insufficient_permissions(user)
}
