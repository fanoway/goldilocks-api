use rocket::{get, launch, routes};

#[get("/v1/areas/<name>")]
fn hello(name: &str) -> String {
    format!("Area -{}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
