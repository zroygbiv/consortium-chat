use rand::prelude::SliceRandom;
use std::{
    env,
    error::Error,
    io::{self, BufRead},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::TcpStream,
};

const LOCAL_ADDR: &str = "127.0.0.1";

/// This includes the chat client implementation
///
/// It establishes a connection with the server at a specified address and port, enabling chat communication. 
/// It generates a random emoji and username for the client, and sends a join message to the server. It then 
/// listens for incoming messages from the server and sends user input messages to the server until the user 
/// enters "quit".
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = env::args().nth(1).unwrap_or_else(|| "8080".to_string());
    let server_addr = format!("{}:{}", LOCAL_ADDR, port);
    // Connect to TCP server at specified address
    let stream = match TcpStream::connect(&server_addr).await {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("❌Failed to connect to the server! No server running on the requested port.❌");
            return Err(err.into())
        }
    };
    // Split TCP stream into reader and writer
    let (reader, mut writer) = stream.into_split();
    // Wrap reader with buffered reader for improved performance
    let mut reader = tokio::io::BufReader::new(reader);

    // Generate random emoji and username for client
    let username = generate_random_username();
    let emoji = generate_random_emoji();

    // Clone emoji and username; maintain proper ownership
    let user_clone = username.clone();
    let emoji_clone = emoji.clone();

    welcome_message(emoji_clone, user_clone);

    // Spawn task to read messages from server
    tokio::spawn(async move {
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

    // Send user joined message
    let join_chat = user_enter_chat(&emoji, &username);
    // Send join message to server
    writer.write_all(join_chat.as_bytes()).await?;
    // Send newline character to indicate end of message
    writer.write_all(b"\n").await?;

    // Read user input from stdin and send it to the server
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        if let Some(Ok(line)) = lines.next() {
            if line.trim().to_lowercase() == "quit" {
                // Send user left chat message
                writer
                    .write_all(user_left_chat(&emoji, &username).as_bytes())
                    .await?;
                writer.write_all(b"\n").await?;
                // Exit the loop if the user enters "quit"
                break;
            }

            // Format the user's message
            let message = format!("{} {}: {}", emoji, username, line);
            // Send the message to the server
            writer.write_all(message.as_bytes()).await?;
            // Send a newline character to indicate the end of the message
            writer.write_all(b"\n").await?;
        }
    }

    // Return Ok to indicate successful execution
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
        "🍪", "🌆", "🐙", "💫", "🎵", "🍿", "🥁", "🗝️", "🌖", "🍨", "🌉", "🦀", "🎶", "🥤", "🎼",
        "🔒", "🌗", "🌤️", "🍦", "🏞️", "🐌", "🌩️", "🎵", "🍺", "🪕", "🐝", "🌘", "🌥️", "🍩", "🏙️", "☀️",
    ];

    let mut rng = rand::thread_rng();
    // Choose random emoji from list
    (*emoji_list.choose(&mut rng).unwrap()).to_string()
}

/// Prints a welcome message with the provided emoji and username.
pub fn welcome_message(emoji: String, username: String) {
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

    // Defines EMOJI_LIST as a static slice (&[&str]), initializes it with the
    // array literal syntax. Each emoji is represented as a string literal (&str)
    // within the array.
    static EMOJI_LIST: &[&str] = &[
        "🌟", "🚀", "💡", "🔥", "🌈", "🐢", "🌺", "🌊", "🎉", "🍕", "🎸", "📚", "🌙", "⚡", "🍦",
        "🌸", "🌞", "🐳", "🌼", "🎻", "🎁", "🍔", "🎹", "🔒", "🌍", "🌩", "🍭", "🌹", "🌄", "🐬",
        "🌻", "💧", "🎈", "🌮", "🌹", "🔑", "🌎", "🌪", "🍩", "🌷", "🌅", "🦈", "🌧", "🎊", "🍟",
        "🎷", "🔓", "🌏", "⛈", "🍰", "🌇", "🐠", "🌺", "💨", "🎀", "🌭", "🎺", "🔐", "🌕", "🌧",
        "🍪", "🌆", "🐙", "💫", "🎵", "🍿", "🥁", "🗝️", "🌖", "🍨", "🌉", "🦀", "🎶", "🥤", "🎼",
        "🔒", "🌗", "🌤️", "🍦", "🏞️", "🐌", "🌩️", "🎵", "🍺", "🪕", "🐝", "🌘", "🌥️", "🍩", "🏙️", "☀️",
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
        assert_eq!(message, "🦀 🌟 test_user has left the chat 🦀");
    }
}
