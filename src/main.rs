#[macro_use]
extern crate rocket;

mod apple_script;
mod media;
mod ui;

#[launch]
fn rocket() -> _ {
    let configuration = rocket::Config::figment()
        .merge(("port", 8050))
        .merge(("address", "0.0.0.0"));
    rocket::custom(configuration).mount("/", routes![ui::index, media::endpoints])
}
