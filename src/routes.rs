use actix_web::{ web };
use crate::controllers::estimationController;


pub fn estimation_request(config: &mut web::ServiceConfig) {
    config.service(web::resource("/estimation").route(web::post().to(estimationController::create)));
}