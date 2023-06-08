use rand::prelude::SliceRandom;
use std::{
    env,
    error::Error,
    io::{self, BufRead},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

const LOCAL_ADDR: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = env::args().nth(1).unwrap_or_else(|| "8080".to_string());
    let addr = format!("{}:{}", LOCAL_ADDR, port);

    // Connect to TCP server at specified address
    let stream = connect_to_server(&addr).await?;

    // Split TCP stream into reader and writer
    let (reader, mut writer) = stream.into_split();
    // Wrap reader with buffered reader for improved performance
    let reader = BufReader::new(reader);
    // Generate random emoji and username for client
    let username = generate_random_username();
    let emoji = generate_random_emoji();

    welcome_message(&emoji, &username);

    // Spawn task to read messages from server
    tokio::spawn(read_messages_from_server(reader));

    // Send user join message to server
    writer
        .write_all(user_enter_chat(&emoji, &username).as_bytes())
        .await?;
    // Send newline character to indicate end of message
    writer.write_all(b"\n").await?;

    // Read user input from stdin, send it to the server
    let stdin = io::stdin();
    // Aquire lock on input stream, set up iterator over input
    let mut lines = stdin.lock().lines();
    // Handle client input, sending messages to server
    send_messages_to_server(&mut lines, &mut writer, &emoji, &username).await?;

    Ok(())
}

/// Connects to a TCP server at the specified address or returns error if connection fails.
async fn connect_to_server(server_addr: &str) -> Result<TcpStream, Box<dyn Error>> {
    match TcpStream::connect(server_addr).await {
        Ok(stream) => Ok(stream),
        Err(err) => {
            eprintln!(
                "❌Failed to connect to the server! No server running on the requested port.❌"
            );
            Err(err.into())
        }
    }
}

/// Reads messages from the server and prints them to the console.
async fn read_messages_from_server(
    mut reader: tokio::io::BufReader<tokio::net::tcp::OwnedReadHalf>,
) {
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line).await {
            // Server connection closed; exit loop
            Ok(0) => break,
            Ok(_) => {
                let message = line.trim();
                // Print received message
                println!("{}", message);
                line.clear();
            }
            // Server read error; exit loop
            Err(_) => break,
        }
    }
}

/// Sends messages from the client to the server.
async fn send_messages_to_server(
    lines: &mut io::Lines<io::StdinLock<'_>>,
    writer: &mut (impl AsyncWriteExt + Unpin),
    emoji: &str,
    username: &str,
) -> Result<(), Box<dyn Error>> {
    loop {
        if let Some(Ok(line)) = lines.next() {
            // User entered "quit"; exit loop
            if line.trim().to_lowercase() == "quit" {
                writer
                    // Send user left chat message to server
                    .write_all(user_left_chat(emoji, username).as_bytes())
                    .await?;
                writer.write_all(b"\n").await?;
                break;
            }
            // Format user's message
            let message = format!("{} {}: {}", emoji, username, line);
            // Send message to the server
            writer.write_all(message.as_bytes()).await?;
            // Send newline character to indicate end of the message
            writer.write_all(b"\n").await?;
        }
    }
    Ok(())
}

/// Generates a random username by choosing two words from a predefined word list
pub fn generate_random_username() -> String {
    let word_list = vec![
        "abyss",
        "almond",
        "amethyst",
        "blossom",
        "blaze",
        "butterfly",
        "cactus",
        "caramel",
        "cascade",
        "diamond",
        "delight",
        "dusk",
        "effervescent",
        "emerald",
        "enigma",
        "falcon",
        "feather",
        "frost",
        "galaxy",
        "gazelle",
        "glimmer",
        "hazel",
        "harmony",
        "hurricane",
        "illusion",
        "indigo",
        "ivory",
        "jade",
        "jewel",
        "jubilee",
        "kaleidoscope",
        "karma",
        "koala",
        "labyrinth",
        "lighthouse",
        "luna",
        "mimosa",
        "mist",
        "mystic",
        "nebula",
        "nectar",
        "oasis",
        "opal",
        "onyx",
        "paradise",
        "peony",
        "penguin",
        "quartz",
        "quasar",
        "quench",
        "radiance",
        "raven",
        "ruby",
        "sapphire",
        "serene",
        "serenity",
        "thunder",
        "triumph",
        "twilight",
        "universe",
        "urchin",
        "utopia",
        "velvet",
        "vivid",
        "vortex",
        "whisper",
        "whisper",
        "xanadu",
        "xenon",
        "yoga",
        "yonder",
        "zeppelin",
        "zenith",
    ];

    let mut rng = rand::thread_rng();
    // Choose two words from list; join with an underscore
    let username: String = (0..2)
        .map(|_| *word_list.choose(&mut rng).unwrap())
        .collect::<Vec<&str>>()
        .join("_");

    username
}

/// Generates a random emoji by choosing one emoji from a predefined emoji list.
pub fn generate_random_emoji() -> String {
    let emoji_list = vec![
        "🌟", "🚀", "💡", "🔥", "🌈", "🐢", "🌺", "🌊", "🎉", "🍕", "🎸", "📚", "🌙", "⚡", "🍦",
        "🌸", "🌞", "🐳", "🌼", "🎻", "🎁", "🍔", "🎹", "🔒", "🌍", "🌩", "🍭", "🌹", "🌄", "🐬",
        "🌻", "💧", "🎈", "🌮", "🌹", "🔑", "🌎", "🌪", "🍩", "🌷", "🌅", "🦈", "🌧", "🎊", "🍟",
        "🎷", "🔓", "🌏", "⛈", "🍰", "🌇", "🐠", "🌺", "💨", "🎀", "🌭", "🎺", "🔐", "🌕", "🌧",
        "🍪", "🌆", "🐙", "💫", "🎵", "🍿", "🥁", "🗝️", "🌖", "🍨", "🌉", "🔒", "🎶", "🥤", "🎼",
        "🌗", "🌤️", "🍦", "🏞️", "🐌", "🌩️", "🎵", "🍺", "🪕", "🐝", "🌘", "🌥️", "🍩", "🏙️", "☀️",
    ];

    let mut rng = rand::thread_rng();
    // Choose random emoji from list
    (*emoji_list.choose(&mut rng).unwrap()).to_string()
}

/// Prints a welcome message with the provided emoji and username.
pub fn welcome_message(emoji: &str, username: &str) {
    println!("\n");
    println!("╔═════════════════════════════════╗");
    println!("║     🦀Consortium Chat v1.0🦀    ║");
    println!("╚═════════════════════════════════╝");
    println!("Welcome to the chat!");
    println!("Your username is: {} {}", emoji, username);
    println!("Enter 'quit' to leave the chat");
    println!("\n");
}

/// Generates a message when a user enters the chat with the provided emoji and username.
pub fn user_enter_chat(emoji: &str, username: &str) -> String {
    format!("🦀::: {} {} has entered the chat :::🦀", emoji, username)
}

/// Generates a message when a user leaves the chat with the provided emoji and username.
pub fn user_left_chat(emoji: &str, username: &str) -> String {
    format!("🦀::: {} {} has left the chat :::🦀", emoji, username)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EMOJI_LIST: &[&str] = &[
        "🌟", "🚀", "💡", "🔥", "🌈", "🐢", "🌺", "🌊", "🎉", "🍕", "🎸", "📚", "🌙", "⚡", "🍦",
        "🌸", "🌞", "🐳", "🌼", "🎻", "🎁", "🍔", "🎹", "🔒", "🌍", "🌩", "🍭", "🌹", "🌄", "🐬",
        "🌻", "💧", "🎈", "🌮", "🌹", "🔑", "🌎", "🌪", "🍩", "🌷", "🌅", "🦈", "🌧", "🎊", "🍟",
        "🎷", "🔓", "🌏", "⛈", "🍰", "🌇", "🐠", "🌺", "💨", "🎀", "🌭", "🎺", "🔐", "🌕", "🌧",
        "🍪", "🌆", "🐙", "💫", "🎵", "🍿", "🥁", "🗝️", "🌖", "🍨", "🌉", "🔒", "🎶", "🥤", "🎼",
        "🌗", "🌤️", "🍦", "🏞️", "🐌", "🌩️", "🎵", "🍺", "🪕", "🐝", "🌘", "🌥️", "🍩", "🏙️", "☀️",
    ];

    #[test]
    fn test_generate_random_username() {
        let username = generate_random_username();
        assert_eq!(username.split("_").count(), 2);
    }

    #[test]
    fn test_generate_random_emoji() {
        let emoji = generate_random_emoji();
        assert!(EMOJI_LIST.contains(&emoji.as_str()));
    }

    #[test]
    fn test_user_enter_chat() {
        let emoji = "🌟";
        let username = "test_user";
        let message = user_enter_chat(emoji, username);
        assert_eq!(message, "🦀::: 🌟 test_user has entered the chat :::🦀");
    }

    #[test]
    fn test_user_left_chat() {
        let emoji = "🌟";
        let username = "test_user";
        let message = user_left_chat(emoji, username);
        assert_eq!(message, "🦀::: 🌟 test_user has left the chat :::🦀");
    }
}
