// use newsletter::main;

fn spawn_app() {
	let srv = newsletter::run().expect("failed to bind server address");
	let _ = tokio::spawn(srv);
}

#[tokio::test]
async fn dummy_test() {
	spawn_app();
	let client = reqwest::Client::new();
	let response = client
		.get("http://localhost:8000/health")
		.send()
		.await
		.expect("failed to execute request");

	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}
