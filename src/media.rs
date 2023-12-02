#![allow(unused_attributes)]

use crate::apple_script::trigger_keys;
use rocket::{get, http::Status};

#[get("/media/<sub_path>")]
pub async fn endpoints(sub_path: &str) -> Status {
    match sub_path {
        "open" => {
            trigger_keys("CMD+DOWN");
            Status::Ok
        }
        "close" => {
            trigger_keys("CMD+Q");
            Status::Ok
        }
        "maximize" => {
            trigger_keys("F");
            Status::Ok
        }
        "pause" => {
            trigger_keys("SPACE");
            Status::Ok
        }
        "volumeup" => {
            trigger_keys("UP");
            Status::Ok
        }
        "volumedown" => {
            trigger_keys("DOWN");
            Status::Ok
        }
        "forward" => {
            trigger_keys("RIGHT");
            Status::Ok
        }
        "backward" => {
            trigger_keys("LEFT");
            Status::Ok
        }
        _ => Status::NotFound,
    }
}
