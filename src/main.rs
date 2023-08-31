/*
 * Log sorcerer, making magic from your logs.
 *
 * The log sorcerer is a personal log magician, looking to provide context
 * and understanding to cryptic client side logs. The sorcerer was born in
 * the video game world, where QA testers attach dense log files to tickets.
 * Everything the game did at that point is in that log file, but you have
 * be a mystical being to parse a 15mb log file and truely understand what
 * went wrong. The sorcerer tries to solve that for you.
 *
 * At its core its grep and sed. The goal is to have this split in 4 parts
 *
 * 1. Core
 * 2. Persistance
 * 3. Understanding
 * 4. Presentation
 *
 * Core takes input in the form of seach parameters and data. It then
 * outputs matching lines.
 *
 * Persistance interfaces with the core, but it has memory of searches.
 * The user can save search queries and results. The goal here is to
 * have a small data base we can run common searches on files give.
 * Example would be this layer has a stored query string that knows how
 * to grab the entire matchmaking logs from the client
 *
 * Understanding is going to be the most difficult. Taking the persistance
 * and core layers, build an idea of what happened in this log. It would
 * run multiple queries and can parse specific lines for key moments. You
 * should be able to get the time and exact line the player logged in,
 * connected to the server, spawn as what team.
 *
 * Presentation is how is this all displayed? I would like platform specific
 * layers. So a swift frontend for mac, .NET for windows etc. This is the
 * last thing to be built as everything up until this point will have a
 * cli interface.
 *
 */
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Parse {
        /// lists test values
        #[arg(short, long)]
        pattern: String,

        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Parse { pattern, file }) => {
            println!("Parsing file {} with pattern {}", file, pattern);
        }
        None => {}
    }
}
