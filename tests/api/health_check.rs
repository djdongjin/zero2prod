use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(resp.status().is_success());
    print!("{:?}", resp.content_length());
    assert_eq!(Some(0), resp.content_length());
}
