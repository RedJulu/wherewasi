use chrono::{DateTime, Utc};
use colored::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Table};

use crate::data_loading::{self, WWIData, WWINote};

pub fn add_note(data: &mut WWIData, text: String, sticky: bool) {
    let id = (data.notes.len() + 1) as u32;

    data.notes.push(WWINote {
        id,
        text,
        created: Utc::now(),
        sticky,
        dismissed: false,
    });
}

pub fn remove_notes(data: &mut WWIData, id: Option<u32>) -> usize {
    let before = data.notes.len();

    data.notes.retain(|n| !id.is_none_or(|i| n.id == i));

    before - data.notes.len()
}

pub fn dismiss_notes(data: &mut WWIData, id: Option<u32>) -> usize {
    let mut count = 0;
    for n in data.notes.iter_mut() {
        if id.is_none_or(|i| n.id == i) && !n.dismissed {
            n.dismissed = true;
            count += 1;
        }
    }
    count
}

pub fn ago(then: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let d = now - then;
    if d.num_days() >= 1 {
        format!("{} days ago", d.num_days())
    } else if d.num_hours() >= 1 {
        format!("{} hours ago", d.num_hours())
    } else {
        format!("{} minutes ago", d.num_minutes().max(0))
    }
}

pub fn show(data: &WWIData, all: bool, full_date: bool, is_enter: bool) {
    let notes: Vec<&WWINote> = data
        .notes
        .iter()
        .filter(|n| !is_enter || n.sticky)
        .filter(|n| all || !n.dismissed)
        .collect();

    if notes.is_empty() {
        let show_info = if let Ok(settings) = data_loading::load_settings() {
            settings.show_info
        } else {
            println!("Settings could not be loaded!");
            true
        };

        if is_enter && !show_info {
            return;
        }

        println!("{} No notes for this folder.", "INFO".blue().bold());
        return;
    }

    println!(
        "\n     {} {}\n",
        "🛌 WHEREWASI".blue().bold(),
        format!("({} Notes)", notes.len()).bright_black(),
    );

    let mut table = Table::new();
    table
        .load_style(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("ID")
            .fg(Color::Cyan)
            .set_alignment(CellAlignment::Center),
        Cell::new("Text")
            .fg(Color::Yellow)
            .set_alignment(CellAlignment::Center),
        Cell::new("Created")
            .fg(Color::Magenta)
            .set_alignment(CellAlignment::Center),
        Cell::new("Manual")
            .fg(Color::Green)
            .set_alignment(CellAlignment::Center),
        Cell::new("Dismissed")
            .fg(Color::Red)
            .set_alignment(CellAlignment::Center),
    ]);

    for n in notes {
        let manual_display = if !n.sticky { "Yes" } else { "No" };
        let manual_color = if !n.sticky { Color::Green } else { Color::Red };

        let dismissed_display = if n.dismissed { "Yes" } else { "No" };
        let dismissed_color = if n.dismissed {
            Color::Green
        } else {
            Color::Red
        };
        let ago_display = if full_date {
            n.created.format("%Y-%m-%d %H:%M:%S").to_string()
        } else {
            ago(n.created, Utc::now())
        };

        table.add_row(vec![
            Cell::new(n.id)
                .fg(Color::White)
                .set_alignment(CellAlignment::Center),
            Cell::new(&n.text).fg(Color::Yellow),
            Cell::new(&ago_display)
                .fg(Color::Magenta)
                .set_alignment(CellAlignment::Center),
            Cell::new(manual_display)
                .fg(manual_color)
                .set_alignment(CellAlignment::Center),
            Cell::new(dismissed_display)
                .fg(dismissed_color)
                .set_alignment(CellAlignment::Center),
        ]);
    }

    println!("{table}\n");
    if !is_enter {
        println!("{} {}\n", "🛌 »".bright_black(), invoked_command().yellow());
    } else {
        println!(
            "{} {}\n",
            "🛌 »".bright_black(),
            "See all with 'wherewasi'".yellow()
        );
    }
}

fn invoked_command() -> String {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        "wherewasi".to_string()
    } else {
        format!("wherewasi {}", args.join(" "))
    }
}
