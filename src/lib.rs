use std::io::{self, BufRead};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

mod util;
use util::*;

const ADDR: &str = "127.0.0.1:8080";

/// Starts the chat server that listens for incoming connections and broadcasts messages to clients
pub async fn start_chat_server() {
    // Bind TCP listener to the specified address and port.
    let listener = match TcpListener::bind(ADDR).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Failed to bind TCP listener: {}", err);
            return;
        }
    };

    server_startup_banner(ADDR);

    // Broadcast channel for sending messages
    let (tx, _rx) = broadcast::channel(100);

    loop {
        // Accept incoming connection from client
        let (mut socket, addr) = listener.accept().await.unwrap();

        // Clone transmitter for spawned task to send messages
        let tx = tx.clone();
        // Receiver subscribes to transmitter to receive messages
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            // Split socket into reader and writer halves
            let (reader, mut writer) = socket.split();

            // Wrap reader in BufReader for convenience
            let mut reader = tokio::io::BufReader::new(reader);

            // Mutable string to store each line read from the client
            let mut line = String::new();

            loop {
                tokio::select! {
                    // Read line from client
                    result = reader.read_line(&mut line) => {
                        let bytes_read = match result {
                            Ok(bytes_read) => bytes_read,
                            Err(err) => {
                                eprintln!("Read error: {}", err);
                                break;
                            }
                        };

                        if bytes_read == 0 {
                            break;
                        }
                        // Send received line and client's address to other clients
                        if let Err(err) = tx.send((line.clone(), addr)) {
                            eprintln!("Send error: {}", err);
                            break;
                        }

                        line.clear();
                    }
                    // Receive message from other clients
                    result = rx.recv() => {
                        let (msg, other_addr) = match result {
                            Ok(msg) => msg,
                            Err(err) => {
                                eprintln!("Receive error: {}", err);
                                break;
                            }
                        };

                        if addr != other_addr {
                            if let Err(err) = writer.write_all(msg.as_bytes()).await {
                                eprintln!("Write error: {}", err);
                                break;
                            }
                        }
                    }
                }
            }
        });
    }
}

/// Starts the chat client and connects to the chat server at the specified address.
pub async fn start_chat_client() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to TCP server at specified address
    let stream = TcpStream::connect(ADDR).await?;
    // Split TCP stream into reader and writer
    let (reader, mut writer) = stream.into_split();
    // Create buffered reader
    let mut reader = tokio::io::BufReader::new(reader);

    // Generate random emoji and username for client
    let username = generate_random_username();
    let emoji = generate_random_emoji();

    // Clone emoji and username; maintain proper ownership
    let user_clone = username.clone();
    let emoji_clone = emoji.clone();
    welcome_message(emoji_clone, user_clone);

    // Spawn task to read messages from server
    let reader_task = tokio::spawn(async move {
        let mut line = String::new();
        loop {
            match reader.read_line(&mut line).await {
                // Connection closed; exit loop
                Ok(0) => break,
                Ok(_) => {
                    let message = line.trim();
                    if !message.is_empty() {
                        // Print received message
                        println!("{}", message);
                    }
                    line.clear();
                }
                // Server read error; exit loop
                Err(_) => break,
            }
        }
    });

    // Send join message
    let join_chat = user_enter_chat(&emoji, &username);
    // Send the join message to the server
    writer.write_all(join_chat.as_bytes()).await?;
    // Send a newline character to indicate the end of the message
    writer.write_all(b"\n").await?;

    // Read user input from stdin and send it to the server
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        if let Some(Ok(line)) = lines.next() {
            // Format the user's message
            let message = format!("{} {}: {}", emoji, username, line);
            // Send the message to the server
            writer.write_all(message.as_bytes()).await?;
            // Send a newline character to indicate the end of the message
            writer.write_all(b"\n").await?;
        } else {
            // Exit the loop if there are no more lines to read from stdin
            break;
        }
    }

    writer
        .write_all(user_left_chat(&emoji, &username).as_bytes())
        .await?;
    writer.write_all(b"\n").await?;

    // Wait for the reader task to complete
    reader_task.await?;

    // Return Ok to indicate successful execution
    Ok(())
}
