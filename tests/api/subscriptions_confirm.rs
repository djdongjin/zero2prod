use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmation_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let resp = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .expect("Failed to execute request.");

    assert_eq!(resp.status().as_u16(), 400);
}
