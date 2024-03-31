use reqwest::Url;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmation_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let resp = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .expect("Failed to execute request.");

    assert_eq!(resp.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Mock: if email server receives POST, return 200.
    // This makes sure our confirm indeed sends an email to the email server (here, mocked).
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act 1: send a POST request to /subscriptions
    // The response body should contain a link to confirm the subscription.
    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();
    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };

    // Extract the confirmation link
    let raw_confirmation_link = &get_link(&body["HtmlBody"].as_str().unwrap());
    let mut confirmation_link = Url::parse(raw_confirmation_link).unwrap();
    // Let's make sure we don't call random APIs on the web
    assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");

    // Act 2: call the returned link should send a GET to /subscriptions/confirm
    // Let's rewrite the URL to include the port first
    confirmation_link.set_port(Some(app.port)).unwrap();
    let resp = reqwest::get(confirmation_link).await.unwrap();

    // Assert
    assert_eq!(resp.status().as_u16(), 200);
}
