use rocket::{get, response::Redirect, routes, uri};

#[get("/")]
fn index() -> &'static str {
    "Hello, bird!"
}

#[get("/-1/seek")]
fn seek() -> Redirect {
    Redirect::found(uri!("https://www.youtube.com/watch?v=9Gc4QTqslN4"))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, seek]);

    Ok(rocket.into())
}
