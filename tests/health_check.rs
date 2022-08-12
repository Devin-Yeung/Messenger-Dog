use std::net::TcpListener;

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

// Launch our application in the background
fn spawn_app() -> String {
    // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an available port which will then be bound to the application.
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port");
    // retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = MessengerDog::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
