use newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}

// #[cfg(test)]
// mod tests {
//     use crate::health_check;
//     #[tokio::test]
//     async fn health_check_succeeds() {
//         let response = health_check().await;
//         assert!(response.status().is_success())
//     }
// }
