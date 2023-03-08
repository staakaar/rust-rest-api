use std::sync::Mutex;

use actix_web::{ web, HttpRequest };
use crate::controllers::estimationController;


pub fn estimation_request(config: &mut web::ServiceConfig) {
    config.service(web::resource("/estimate/{code}")
            .route(web::get().to(estimationController::index)))
            .service(web::resource("/estimate").route(web::post().to(estimationController::EstimationController::create(&self))));
}