use rocket::{get, post};
use rocket::response::{Redirect, Responder};
use rocket_dyn_templates::{context, Template};

#[get("/login")]
pub async fn login() -> Template {
    Template::render(
        "login",
        context! {
            title: "Login"
        },
    )
}

#[post("/sign-in")]
pub async fn authenticate() ->  Result<Redirect, Template> {
    Ok(Redirect::to("/"))
}
