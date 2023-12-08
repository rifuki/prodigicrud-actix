use actix_web::{Responder, HttpResponse, get};
use serde_json::json;

/// Get `ping` API Response
/// 
/// Healthy API should always response OK
#[utoipa::path(
    get,
    path = "/api/ping",
    tag = "ping",
    responses(
        (status = 200, description = "Success"),
        (status = 500, description = "Server Error.")
    )
)]
#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok()
        .json(json!({
            "msg": "PONG"
        }))
} 