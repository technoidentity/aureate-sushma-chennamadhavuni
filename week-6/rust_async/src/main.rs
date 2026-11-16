async fn fetch_user() -> String {
    "Sushma".to_string()
}
#[tokio::main]
async fn main() {
    let user = fetch_user().await;
    println!("Fetched user successfully: {}", user);
}
