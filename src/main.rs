#[macro_use]
extern crate rocket;

mod apple_script;
mod media;
mod ui;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ui::index, media::endpoints])
}
