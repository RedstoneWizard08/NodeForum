use actix_web::{HttpResponse, get};

#[get("/")]
pub async fn index() -> HttpResponse {
    return HttpResponse::Ok().body("Hello, world!");
}
