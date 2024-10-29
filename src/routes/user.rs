use actix_web::{post, HttpResponse};

#[post("/api/register")]
pub async fn register_user() -> HttpResponse {
    HttpResponse::Ok().json("User registered")
}
