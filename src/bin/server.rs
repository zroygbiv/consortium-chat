use std::{env, error::Error, io::Write, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::{broadcast, Mutex},
};
const LOCAL_ADDR: &str = "127.0.0.1";

/// This contains the chat server implementation using the Tokio runtime for asynchronous networking.
///
/// It binds a TCP listener to a specified address and port, accepts incoming connections from clients,
/// and enables communication between clients by broadcasting messages to all connected clients.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Bind TCP listener to the specified address and port.
    let port = env::args().nth(1).unwrap_or_else(|| "8080".to_string());
    let addr = format!("{}:{}", LOCAL_ADDR, port);

    let listener = TcpListener::bind(&addr).await?;

    println!("{}", server_startup_banner(&addr));

    // Broadcast channel for sending messages (transmitter & receiver)
    let (tx, _rx) = broadcast::channel(500);
    // Wrap transmitter in ARC and mutex for synchronized sharing
    let tx_arc = Arc::new(Mutex::new(tx));

    // Spawn task to listen for CLI shutdown command
    tokio::spawn(async move {
        let mut input = String::new();
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);

        loop {
            print!("\n🦀> ");
            std::io::stdout().flush().unwrap(); // Flush the stdout to ensure prompt is displayed

            if reader.read_line(&mut input).await.unwrap() == 0 {
                break; 
            }

            let cmd = input.trim().to_lowercase();

            if cmd == "shutdown" {
                println!("Shutting down the server...");
                // Terminate 
                std::process::exit(0);
            } else {
                println!("Unknown command. Please enter 'shutdown' to stop the server.");
            }

            input.clear();
        }
    });

    loop {
        // Accept incoming connection from client
        let (mut socket, addr) = listener.accept().await?;
        // Clone Arc<Mutex> for spawned task to send messages
        let tx = Arc::clone(&tx_arc);
        // Wrap transmitter in ARC and mutex for synchronized sharing
        let mut rx = tx.lock().await.subscribe();

        tokio::spawn(async move {
            // Split socket into reader and writer halves
            let (reader, mut writer) = socket.split();
            // Wrap reader in BufReader for convenience
            let mut reader = tokio::io::BufReader::new(reader);
            // Mutable string to store each line read from the client
            let mut line = String::new();

            loop {
                // Handle async operations concurrently
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
                         if let Err(err) = tx.lock().await.send((line.clone(), addr)) {
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

/// Prints a startup banner for the server with the provided address.
pub fn server_startup_banner(addr: &str) -> String {
    let mut output = String::new();
    output.push_str("    ╔══════════════════════════════════════════════╗\n");
    output.push_str(&format!(":::: \x1b[91mConsortium Server\x1b[0m is \x1b[92mOnline\x1b[0m on \x1b[94;1m{}\x1b[0m  ::::\n", addr));
    output.push_str("    ╚══════════════════════════════════════════════╝\n");
    output.push_str("Enter 'shutdown' to quit the server:");

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_startup_banner() {
        let addr = "127.0.0.1:8080";
        let expected_output = format!(
            "    ╔══════════════════════════════════════════════╗\n\
            :::: \x1b[91mConsortium Server\x1b[0m is \x1b[92mOnline\x1b[0m on \x1b[94;1m{}\x1b[0m  ::::\n    \
                 ╚══════════════════════════════════════════════╝\n\
            Enter 'shutdown' to quit the server:",
            addr
        );

        let output = server_startup_banner(addr);
        assert_eq!(output, expected_output);
    }
}
