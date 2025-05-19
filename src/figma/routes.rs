use actix_web::web;
use crate::figma::get_file;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file::get_file);
}