#![allow(unused_attributes)]

use crate::apple_script::trigger_keys;
use rocket::{get, http::Status};

#[get("/media/<sub_path>")]
pub async fn endpoints(sub_path: String) -> Status {
    match sub_path.as_str() {
        "maximize" => {
            trigger_keys(vec!["F"]);
            Status::Ok
        }
        "pause" => {
            trigger_keys(vec!["SPACE"]);
            Status::Ok
        }
        "volumeup" => {
            trigger_keys(vec!["UP"]);
            Status::Ok
        }
        "volumedown" => {
            trigger_keys(vec!["DOWN"]);
            Status::Ok
        }
        "forward" => {
            trigger_keys(vec!["RIGHT"]);
            Status::Ok
        }
        "backward" => {
            trigger_keys(vec!["LEFT"]);
            Status::Ok
        }
        _ => Status::NotFound,
    }
}
