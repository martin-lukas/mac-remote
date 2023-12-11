#![allow(unused_attributes)]

use crate::commands::trigger_keys;
use rocket::{get, http::Status};

#[get("/media/<sub_path>")]
pub async fn endpoints(sub_path: &str) -> Status {
    match sub_path {
        "lefttab" => {
            trigger_keys("OPT+CMD+LEFT");
            Status::Ok
        }
        "righttab" => {
            trigger_keys("OPT+CMD+RIGHT");
            Status::Ok
        }
        "closetab" => {
            trigger_keys("CMD+W");
            Status::Ok
        }
        "changewindow" => {
            trigger_keys("OPT+`");
            Status::Ok
        }
        "openfile" => {
            trigger_keys("CMD+DOWN");
            Status::Ok
        }
        "closewindow" => {
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
