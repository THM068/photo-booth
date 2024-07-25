use rocket::response::stream::TextStream;
use rocket::tokio::time::{self, Duration};
use rocket::{catch, get, Request};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    let names = vec!["Alice", "Bob", "Charlie"];
    Template::render(
        "index",
        context! {
            title: "Home",
            names: names
        },
    )
}

#[get("/about")]
pub fn about() -> Template {
    Template::render(
        "about",
        context! {
            title: "About"
        },
    )
}

#[get("/url-shortener")]
pub fn url_shortener() -> Template {
    Template::render(
        "url-shortner",
        context! {
            title: "URL Shortener"
        },
    )
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[get("/infinite-hellos")]
pub fn streamer() -> TextStream![&'static str] {
    TextStream! {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            yield "hello";
            interval.tick().await;
        }
    }
}
