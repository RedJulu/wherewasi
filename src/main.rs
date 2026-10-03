use anyhow::{Result, bail};
use clap::Parser;

use cli::{Cli, Commands};

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
fn main() -> Result<()> {
    let args = Cli::parse();

    if let Some(Commands::Init { shell }) = &args.command {
        match shell.as_str() {
            "zsh" => println!("{ZSH_HOOK}"),
            "bash" => println!("{BASH_HOOK}"),
            other => bail!("Unsupported shell: {other} | (Only zsh & bash)"),
        }
        return Ok(());
    }

    let mut data = data_loading::load_data()?;
    let full_date = args.full_date;

    match args.command {
        None => manager::show(&data, args.all, full_date, false),
        Some(Commands::Note { text, manual }) => {
            manager::add_note(&mut data, text, !manual);
            data_loading::save_data(&data)?;
            println!("Note saved.");
        }
        Some(Commands::Done { id, all }) => {
            let target = if all { None } else { id };
            let n = manager::remove_notes(&mut data, target);
            data_loading::save_data(&data)?;
            println!("Removed {n} note(s).");
        }
        Some(Commands::Dismiss { id, all }) => {
            let target = if all { None } else { id };
            let n = manager::dismiss_notes(&mut data, target);
            data_loading::save_data(&data)?;
            println!("Dismissed {n} note(s).");
        }
        Some(Commands::Enter) => manager::show(&data, false, full_date, true),
        Some(Commands::Init { .. }) => unreachable!(),
    }

    Ok(())
}
