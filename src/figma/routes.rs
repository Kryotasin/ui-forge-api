// Updated routes.rs - using imports instead of module declarations
use actix_web::web;
use crate::figma::echo;
use crate::figma::get_file;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file::get_file)
       .service(echo::echo);
}