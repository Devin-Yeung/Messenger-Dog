use std::net::TcpListener;

// Launch our application in the background
fn spawn_app() -> String {
    // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an available port which will then be bound to the application.
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port");
    // retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = messenger_dog::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
// checking
// • the health check is exposed at /health_check;
// • the health check is behind a GET method;
// • the health check always returns a 200;
// • the health check’s response has no body.
async fn health_check_works() {
    let address = spawn_app();
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    // a tokio runtime is shut down all tasks spawned on it are dropped
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=foo%20bar&email=foobar%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=foo%20bar", "missing the email"),
        ("email=foobar%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (body, hint) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            response.status().as_u16(),
            400, // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            hint
        );
    }
}
