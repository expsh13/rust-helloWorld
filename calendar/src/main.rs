use std::{fs::File, io::BufReader};

use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Calendar {
    schedules: Vec<Schedule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Schedule {
    id: u64,
    subject: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

const SCHEDULE_FILE: &str = "schedule.json";

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
}

fn main() {
    let options = Cli::parse();
    match options.command {
        Commands::List => show_list(),
    }
}

fn show_list() {
    // 予定の読み込み
    let calendar: Calendar = {
        let file = File::open(SCHEDULE_FILE).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };
    // 予定の表示
    println!("ID\tSTART\tEND\tSUBJECT");
    for schedule in calendar.schedules {
        println!(
            "{}\t{}\t{}\t{}",
            schedule.id, schedule.start, schedule.end, schedule.subject
        )
    }
}
