use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct Parameters {
    pub subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(_params))]
pub async fn confirm(_params: web::Query<Parameters>) -> impl Responder {
    HttpResponse::Ok().finish()
}
