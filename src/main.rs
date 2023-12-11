#[macro_use]
extern crate rocket;

mod commands;
mod actions;
mod ui;

#[launch]
fn rocket() -> _ {
    let configuration = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "0.0.0.0"));
    rocket::custom(configuration).mount("/", routes![ui::index, actions::endpoints])
}
