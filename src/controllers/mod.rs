use rocket::http::Status;
use rocket::Responder;
use sea_orm::DbErr;

pub mod bakery;
pub mod contact;
pub mod home;

#[derive(Responder)]
pub struct SuccessResponse<T>(pub (Status, T));

#[derive(Responder)]
pub struct ErrorResponse(pub (Status, String));

pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;

impl From<DbErr> for ErrorResponse {
    fn from(err: DbErr) -> Self {
        ErrorResponse((Status::InternalServerError, err.to_string()))
    }
}
