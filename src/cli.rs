use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::BrightCyan.on_default() | Effects::BOLD)
        .usage(AnsiColor::BrightCyan.on_default() | Effects::BOLD)
        .literal(AnsiColor::BrightRed.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::BrightYellow.on_default())
}

#[derive(Parser, Debug)]
#[command(
    name = "wherewasi",
    author = "RedJulu",
    version = "1.0",
    about = "🛌 Remember what you were coding while you were gone",
    styles = styles(),
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, help = "Shows all notes including dismissed.")]
    pub all: bool,

    #[arg(
        long = "full-date",
        help = "Shows the literal Date instead of relative."
    )]
    pub full_date: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Create a new note. Use --manual to not show it automatically on cd")]
    Note {
        text: String,
        #[arg(long, help = "Don't show this note automatically on cd")]
        manual: bool,
    },
    #[command(about = "Delete a note")]
    Done {
        #[arg(conflicts_with = "all", required_unless_present = "all", value_parser = clap::value_parser!(u32).range(1..))]
        id: Option<u32>,

        #[arg(short, long)]
        all: bool,
    },
    #[command(about = "Silence a note")]
    Dismiss {
        #[arg(conflicts_with = "all", required_unless_present = "all", value_parser = clap::value_parser!(u32).range(1..))]
        id: Option<u32>,

        #[arg(short, long)]
        all: bool,
    },
    #[command(hide = true)]
    Enter,
    #[command(about = "Print the shell hook")]
    Init { shell: String },
    Config {
        #[command(subcommand)]
        commands: ConfigCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    #[command(about = "Set a setting")]
    Set { key: String, value: String },
    #[command(about = "Show current settings")]
    Show,
}
