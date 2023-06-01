use rand::prelude::SliceRandom;

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
    println!("\n");
}

/// Generates a message when a user enters the chat with the provided emoji and username.
pub fn user_enter_chat(emoji: &str, username: &str) -> String {
    format!("🦀::: {} {} has entered the chat :::🦀", emoji, username)
}

/// Generates a message when a user leaves the chat with the provided emoji and username.
pub fn user_left_chat(emoji: &str, username: &str) -> String {
    format!("🦀 {} {} has left the chat 🦀", emoji, username)
}

/// Prints a startup banner for the server with the provided address.
pub fn server_startup_banner(addr: &str) {
    println!("\n");
    println!("    🌐🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🌐");
    println!("    ╔══════════════════════════════════════════════╗");
    println!(":::: \x1b[91mConsortium Server\x1b[0m is \x1b[92mOnline\x1b[0m on \x1b[94;1m{}\x1b[0m  ::::", addr);
    println!("    ╚══════════════════════════════════════════════╝");
    println!("    🌐🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🌐");
}

// pub fn server_shutdown_banner() {
//     println!("\n");
//     println!("    🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀");
//     println!("    ╔══════════════════════════════════════════════╗");
//     println!("::::    \x1b[91mConsortium Server\x1b[0m Shutting Down...    ::::");
//     println!("    ╚══════════════════════════════════════════════╝");
//     println!("    🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀");
// }
