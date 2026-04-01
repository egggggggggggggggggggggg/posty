use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    Ok(())
}
//WRite two backends for the request sending, one with reqwest and one thats custom made. The
//custom one will prob be bad, its more to learn how to implement TLS, request sending etc.
