use actix_web::web;
use crate::figma::get_file;
use crate::figma::get_node;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file::get_file)
    .service(get_node::get_node);
}