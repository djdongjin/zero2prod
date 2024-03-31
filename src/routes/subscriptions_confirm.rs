use actix_web::{HttpResponse, Responder};

#[tracing::instrument(name = "Confirm a pending subscriber")]
pub async fn confirm() -> impl Responder {
    HttpResponse::Ok().finish()
}
