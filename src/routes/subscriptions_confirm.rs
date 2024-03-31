use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct Parameters {
    pub subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(params, db_pool))]
pub async fn confirm(params: web::Query<Parameters>, db_pool: web::Data<PgPool>) -> impl Responder {
    let id = match get_subscriber_id_from_token(&db_pool, &params.subscription_token).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match id {
        None => HttpResponse::Unauthorized().finish(),
        Some(subscriber_id) => {
            if confirm_subscriber(&db_pool, subscriber_id).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().finish()
        }
    }
}

#[tracing::instrument(name = "Mark subcriber as confirmed", skip(id, pool))]
pub async fn confirm_subscriber(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE subscriptions
        SET status = 'confirmed'
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!(
            "Failed to update subscriber status in the database: {:?}",
            e
        );
        e
    })?;
    Ok(())
}

#[tracing::instrument(name = "Retrieving subscriber ID from the database", skip(token, pool))]
async fn get_subscriber_id_from_token(
    pool: &PgPool,
    token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT subscriber_id
        FROM subscription_tokens
        WHERE subscription_token = $1
        "#,
        token,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}
