use actix_web::{ web };
use serde::{Serialize, Deserialize};
use crate::controllers::estimationController::EstimationController;

#[derive(Serialize, Deserialize)]
pub struct EstimateRequest {
    id: i128,
    name: String,
    desired_amount: String
}


pub fn estimation_request(config: &mut web::ServiceConfig) {
    config.service(web::resource("/estimation").route(web::post().to(EstimationController::create)));
}