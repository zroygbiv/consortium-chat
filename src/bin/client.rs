use chat_lib::start_chat_client;

/// Asynchronous main function that starts the chat client.
///
/// Calls the `start_chat_client` function, returns a `Result` indicating success or an error.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_chat_client().await?;
    Ok(())
}
