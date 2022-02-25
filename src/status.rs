use rocket::http::Status;

pub trait ToStatus {
    fn to_status(self) -> Status;
}
