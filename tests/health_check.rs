//! test/health_check.rs
use zero2prod::startup::run;

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind to port");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Subscribe Test
#[actix_rt::test]
async fn subscriber_returns_a_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=le%20lemon&email=le%20lemon%40example.com";

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscriber_returns_a_400_for_invalid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20lemon", "missing the email"),
        ("email=le%20lemon", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
