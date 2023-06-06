use chat_lib::start_chat_server;

/// Asynchronous main function that starts the chat server.
///
/// Calls `start_chat_server` function, internally handles error checking, doesn't return `Result`.
#[tokio::main]
async fn main() {
    start_chat_server().await;
}
