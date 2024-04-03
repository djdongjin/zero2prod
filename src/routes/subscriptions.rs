use actix_web::{web, HttpResponse, ResponseError};
use chrono::Utc;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{
    domain::{NewSubscriber, SubscriberEmail, SubscriberName},
    email_client::EmailClient,
    startup::ApplicationBaseUrl,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let email = SubscriberEmail::parse(value.email)?;
        let name = SubscriberName::parse(value.name)?;

        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber", // span message, fn name by default
    skip(form, db_pool, email_client), // skip these two fields in the span
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> Result<HttpResponse, SubscribeError> {
    // `web::Form` is a wrapper around `FormData`
    // `form.0` gives us access to the underlying `FormData`
    let new_subscriber = form.0.try_into()?;

    let mut txn = db_pool.begin().await?;
    let subscriber_id = insert_subscriber(&new_subscriber, &mut txn).await?;
    let subscription_token = generate_subscription_token();
    store_token(&mut txn, subscriber_id, &subscription_token).await?;
    txn.commit().await?;

    send_confirmation_email(
        new_subscriber,
        &email_client,
        &base_url.0,
        &subscription_token,
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, txn)
)]
async fn insert_subscriber(
    form: &NewSubscriber,
    txn: &mut Transaction<'_, Postgres>,
) -> Result<Uuid, sqlx::Error> {
    let uuid = Uuid::new_v4();
    let query = sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')"#,
        uuid,
        form.email.as_ref(),
        form.name.as_ref(),
        Utc::now()
    );
    txn.execute(query).await.map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(uuid)
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, form, base_url, subscription_token)
)]
pub async fn send_confirmation_email(
    form: NewSubscriber,
    email_client: &EmailClient,
    base_url: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        base_url, subscription_token
    );
    let plain_body = format!(
        "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
        confirmation_link
    );
    let html_body = format!(
        "Welcome to our newsletter!<br />\
                Click <a href=\"{}\">here</a> to confirm your subscription.",
        confirmation_link
    );

    email_client
        .send_email(form.email, "welcome", &html_body, &plain_body)
        .await
}

#[tracing::instrument(
    name = "Storing subscription token in the database",
    skip(txn, subscriber_token)
)]
pub async fn store_token(
    txn: &mut Transaction<'_, Postgres>,
    subscriber_id: Uuid,
    subscriber_token: &str,
) -> Result<(), StoreTokenError> {
    let query = sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscriber_token,
        subscriber_id
    );
    txn.execute(query).await.map_err(|e| StoreTokenError(e))?;
    Ok(())
}

fn generate_subscription_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

#[derive(Debug)]
pub enum SubscribeError {
    ValidationError(String),
    DatabaseError(sqlx::Error),
    StoreTokenError(StoreTokenError),
    SendEmailError(reqwest::Error),
}

impl std::fmt::Display for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A server error occurred while trying to subscribe.")
    }
}

impl std::error::Error for SubscribeError {}

impl ResponseError for SubscribeError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SubscribeError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            SubscribeError::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            SubscribeError::StoreTokenError(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            SubscribeError::SendEmailError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<reqwest::Error> for SubscribeError {
    fn from(e: reqwest::Error) -> Self {
        SubscribeError::SendEmailError(e)
    }
}

impl From<StoreTokenError> for SubscribeError {
    fn from(e: StoreTokenError) -> Self {
        SubscribeError::StoreTokenError(e)
    }
}

impl From<sqlx::Error> for SubscribeError {
    fn from(e: sqlx::Error) -> Self {
        SubscribeError::DatabaseError(e)
    }
}

impl From<String> for SubscribeError {
    fn from(e: String) -> Self {
        SubscribeError::ValidationError(e)
    }
}

// A new error type, wrapping a `sqlx::Error`.
pub struct StoreTokenError(sqlx::Error);

impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nCaused by:\n\t{}", self, self.0)
    }
}

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encounterd while \
            trying to store a subscription token."
        )
    }
}

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // The compiler transparently casts `&sqlx::Error` into a `&dyn Error`
        Some(&self.0)
    }
}
