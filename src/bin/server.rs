use std::{env, error::Error, io::Write, net::SocketAddr, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{broadcast, Mutex},
};

const LOCAL_ADDR: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve port # from CLI arg or default to port 8080
    let port = env::args().nth(1).unwrap_or_else(|| "8080".to_string());
    // Concatenate address and port #
    let addr = format!("{}:{}", LOCAL_ADDR, port);
    // Bind TCP listener to the specified address and port.
    let listener = TcpListener::bind(&addr).await?;

    println!("{}", server_startup_banner(&addr));

    // Broadcast channel for sending messages (transmitter & receiver)
    let (tx, _rx) = broadcast::channel(1000);
    // Wrap transmitter in ARC and mutex for synchronized sharing
    let tx = Arc::new(Mutex::new(tx));

    // Spawn async task to handle server CLI shutdown command
    tokio::spawn(handle_shutdown_command());

    loop {
        // Accept incoming connection from new client; assign accepted socket and remote address
        let (socket, addr) = listener.accept().await?;
        // Ensure each spawned task has own reference to transmitter
        let tx = Arc::clone(&tx);
        // Aquire lock on transmitter, create receiver subscribed to recieve messages from transmitter
        let rx = tx.lock().await.subscribe();

        // Spawn async task to handle communication with connected client
        tokio::spawn(handle_client_communication(
            socket,
            addr,
            Arc::clone(&tx),
            rx,
        ));
    }
}

/// This function reads input from the standard input stream in a loop and checks if the entered command
/// is "shutdown". If so, it prints a message indicating the server shutdown and exits the process; otherwise,
/// it prints an "Unknown command!" message.
async fn handle_shutdown_command() {
    let mut input = String::new();
    // Handle to input stream in async context
    let stdin = tokio::io::stdin();
    // Wrap input stream with async buffered reader
    let mut reader = BufReader::new(stdin);

    loop {
        print!("\n🦀> ");
        std::io::stdout().flush().unwrap();

        if let Err(err) = reader.read_line(&mut input).await {
            eprintln!("Read error: {}", err);
            break;
        }

        let cmd = input.trim().to_lowercase();

        if cmd == "shutdown" {
            println!("Shutting down the server...");
            std::process::exit(0);
        } else {
            println!("Unknown command! Please enter 'shutdown' to stop the server.");
        }

        input.clear();
    }
}

/// Reads lines from the client, broadcasts received messages to other clients, and sends messages
/// from other clients to the current client if their addresses are different. Uses `tokio` for
/// asynchronous I/O operations and handles errors during reading, sending, and writing.
async fn handle_client_communication(
    mut socket: TcpStream,
    addr: SocketAddr,
    tx: Arc<Mutex<broadcast::Sender<(String, SocketAddr)>>>,
    mut rx: broadcast::Receiver<(String, SocketAddr)>,
) {
    // Split socket to allow reading from and writing data to stream
    let (reader, mut writer) = socket.split();
    // Wrap reader in Tokio buffered reader
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        // Concurrently wait for multiple async branches, execute first that becomes ready
        tokio::select! {
            // Read line from client, send received line and client's address to other clients
            result = reader.read_line(&mut line) => {
                let bytes_read = match result {
                    Ok(bytes_read) => bytes_read,
                    Err(err) => {
                        eprintln!("Read error: {}", err);
                        break;
                    }
                };
                // Client closed connection; exit loop
                if bytes_read == 0 {
                    break;
                }
                if let Err(err) = tx.lock().await.send((line.clone(), addr)) {
                    eprintln!("Send error: {}", err);
                    break;
                }
                line.clear();
            }
            // Receive message from another client, send to current client if addresses are different
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
}

/// Prints a startup banner for the server with the provided socket address.
pub fn server_startup_banner(addr: &str) -> String {
    let mut output = String::new();
    output.push_str("    ╔══════════════════════════════════════════════╗\n");
    output.push_str(&format!(":::: \x1b[91mConsortium Server\x1b[0m is \x1b[92mOnline\x1b[0m on \x1b[94;1m{}\x1b[0m  ::::\n", addr));
    output.push_str("    ╚══════════════════════════════════════════════╝\n");
    output.push_str("Enter 'shutdown' to terminate the server:");

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
            Enter 'shutdown' to terminate the server:",
            addr
        );

        let output = server_startup_banner(addr);
        assert_eq!(output, expected_output);
    }
}
