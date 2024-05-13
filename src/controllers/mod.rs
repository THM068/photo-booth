use rocket::http::Status;
use rocket::{FromForm, Responder};
use sea_orm::DbErr;
use serde::Deserialize;

pub mod authentication;
pub mod bakery;
pub mod contact;
pub mod home;
pub mod register;

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

#[derive(FromForm)]
struct User_Form <'r>{
    #[field(validate = len(1..))]
    email: &'r str,
    #[field(validate = len(1..))]
    password: &'r str,
    #[field(validate = eq(self.password).or_else(msg!("Confirm password must match password.")))]
    confirm_password: &'r str,
}
