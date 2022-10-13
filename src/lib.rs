use rocket::response::content::RawHtml;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../signup.html"))
}

#[shuttle_service::main]
async fn rocket() -> shuttle_service::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket)
}
