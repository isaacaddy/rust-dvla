use actix_web::{get, HttpResponse};

#[get("/api/vehicle")]
pub async fn get_vehicle() -> HttpResponse {
    HttpResponse::Ok().json("Vehicle information")
}
