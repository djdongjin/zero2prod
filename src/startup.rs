use crate::routes::{health_check, subscribe};

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use actix_web::middleware::Logger;
use sqlx::PgPool;
use std::net::TcpListener;

// `async` is no longer needed as we don't have .await calls.
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // create a ARC pointer to the DB connection
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
