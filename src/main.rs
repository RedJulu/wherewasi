use anyhow::{Result, bail};
use clap::Parser;
use colored::Colorize;

use cli::{Cli, Commands, ConfigCommands};

mod cli;
mod data_loading;
mod manager;

const ZSH_HOOK: &str = r#"wherewasi_chpwd() { wherewasi enter; }
autoload -U add-zsh-hook
add-zsh-hook chpwd wherewasi_chpwd"#;

const BASH_HOOK: &str = r#"wherewasi_chpwd() {
    wherewasi enter
}

cd() {
    builtin cd "$@" || return
    wherewasi_chpwd
}"#;

const POWERSHELL_HOOK: &str = r#"function wherewasi_chpwd {
    wherewasi enter
}

function Set-Location {
    Microsoft.PowerShell.Management\Set-Location @args
    if ($?) {
        wherewasi_chpwd
    }
}"#;

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Some(Commands::Init { shell }) = &args.command {
        match shell.as_str() {
            "zsh" => println!("{ZSH_HOOK}"),
            "bash" => println!("{BASH_HOOK}"),
            "powershell" => println!("{POWERSHELL_HOOK}"),
            other => bail!("Unsupported shell: {other} (only zsh, bash and powershell for now)"),
        }

        return Ok(());
    }

    if let Some(Commands::Config { commands }) = args.command {
        match commands {
            ConfigCommands::Show => {
                let settings = data_loading::load_settings()?;

                println!();
                println!("{}", "WHEREWASI CONFIG".blue().bold());
                println!();
                println!(
                    "  {} {}",
                    "show_info".bright_black(),
                    if settings.show_info {
                        "true".green()
                    } else {
                        "false".red()
                    }
                );
                println!();
            }

            ConfigCommands::Set { key, value } => {
                let mut settings = data_loading::load_settings()?;

                match key.as_str() {
                    "show_info" => {
                        settings.show_info = match value.as_str() {
                            "true" => true,
                            "false" => false,
                            _ => bail!("Invalid value for show_info: use true or false"),
                        };
                    }
                    _ => bail!("Unknown setting: {key}"),
                }

                data_loading::save_settings(&settings)?;

                println!(
                    "{} {} set to {}.",
                    "✓".green().bold(),
                    key.green(),
                    value.green()
                );
            }
        }

        return Ok(());
    }

    let mut data = data_loading::load_data()?;

    match args.command {
        None => manager::show(&data, args.all, args.full_date, false),

        Some(Commands::Note { text, manual }) => {
            manager::add_note(&mut data, text, !manual);
            data_loading::save_data(&data)?;

            println!("{} {}", "✓".green().bold(), "Note saved.".green());
        }

        Some(Commands::Done { id, all }) => {
            let target = if all { None } else { id };
            let n = manager::remove_notes(&mut data, target);
            data_loading::save_data(&data)?;

            if n == 0 {
                println!(
                    "{} {}",
                    "!".yellow().bold(),
                    "No matching notes found.".yellow()
                );
            } else {
                println!(
                    "{} {}",
                    "✓".green().bold(),
                    format!("Removed {n} note(s).").green()
                );
            }
        }

        Some(Commands::Dismiss { id, all }) => {
            let target = if all { None } else { id };
            let n = manager::dismiss_notes(&mut data, target);
            data_loading::save_data(&data)?;

            if n == 0 {
                println!(
                    "{} {}",
                    "!".yellow().bold(),
                    "No matching active notes found.".yellow()
                );
            } else {
                println!(
                    "{} {}",
                    "✓".green().bold(),
                    format!("Dismissed {n} note(s).").green()
                );
            }
        }

        Some(Commands::Enter) => manager::show(&data, false, false, true),

        Some(Commands::Init { .. }) | Some(Commands::Config { .. }) => unreachable!(),
    }

    Ok(())
}
