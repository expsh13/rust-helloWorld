use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

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
impl Schedule {
    fn intersects(&self, other: &Schedule) -> bool {
        self.start < other.end && other.start < self.end
    }
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
    Add {
        subject: String,
        start: NaiveDateTime,
        end: NaiveDateTime,
    },
}

fn main() {
    let options = Cli::parse();
    match options.command {
        Commands::List => show_list(),
        Commands::Add {
            subject,
            start,
            end,
        } => add_schedule(subject, start, end),
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

fn add_schedule(subject: String, start: NaiveDateTime, end: NaiveDateTime) {
    let mut calendar: Calendar = {
        let file = File::open(SCHEDULE_FILE).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    let id = calendar.schedules.len() as u64;
    let new_schedule = Schedule {
        id,
        subject,
        start,
        end,
    };

    // 重複判定
    for schedule in &calendar.schedules {
        if schedule.intersects(&new_schedule) {
            println!("予定の重複！！");
            return;
        }
    }

    calendar.schedules.push(new_schedule);

    {
        let file = File::create(SCHEDULE_FILE).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &calendar).unwrap();
    }
    println!("予定を追加しました");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn naive_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }

    #[rstest]
    #[case(18, 15, 19, 15, true)]
    #[case(19, 45, 20, 45, true)]
    #[case(18, 30, 20, 15, true)]
    #[case(20, 15, 20, 45, false)]
    #[case(18, 15, 18, 45, false)]
    #[case(19, 15, 19, 45, true)]

    fn test_schedule_intersects(
        #[case] h0: u32,
        #[case] m0: u32,
        #[case] h1: u32,
        #[case] m1: u32,
        #[case] should_intersects: bool,
    ) {
        let schedule = Schedule {
            id: 0,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, h0, m0, 0),
            end: naive_date_time(2024, 1, 1, h1, m1, 0),
        };
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 0, 0),
            end: naive_date_time(2024, 1, 1, 20, 0, 0),
        };
        assert_eq!(should_intersects, schedule.intersects(&new_schedule));
    }

    // // #[test]
    // // fn test_schedule_intersects_1() {
    // //     let schedule = Schedule {
    // //         id: 1,
    // //         subject: "既存予定1".to_string(),
    // //         start: naive_date_time(2024, 1, 1, 18, 15, 0),
    // //         end: naive_date_time(2024, 1, 1, 19, 15, 0),
    // //     };
    // //     let new_schedule = Schedule {
    // //         id: 999,
    // //         subject: "新規予定".to_string(),
    // //         start: naive_date_time(2024, 1, 1, 19, 0, 0),
    // //         end: naive_date_time(2024, 1, 1, 19, 0, 0),
    // //     };
    // //     assert!(schedule.intersects(&new_schedule));
    // // }
    // // #[test]
    // // fn test_schedule_intersects_2() {
    // //     let schedule = Schedule {
    // //         id: 1,
    // //         subject: "既存予定2".to_string(),
    // //         start: naive_date_time(2024, 1, 1, 19, 45, 0),
    // //         end: naive_date_time(2024, 1, 1, 20, 45, 0),
    // //     };
    // //     let new_schedule = Schedule {
    // //         id: 999,
    // //         subject: "新規予定".to_string(),
    // //         start: naive_date_time(2024, 1, 1, 19, 0, 0),
    // //         end: naive_date_time(2024, 1, 1, 20, 0, 0),
    // //     };
    // //     assert!(schedule.intersects(&new_schedule));
    // // }
    // // #[test]
    // // fn test_schedule_intersects_3() {
    // //     let schedule = Schedule {
    // //         id: 1,
    // //         subject: "既存予定3".to_string(),
    // //         start: naive_date_time(2024, 1, 1, 18, 30, 0),
    // //         end: naive_date_time(2024, 1, 1, 20, 25, 0),
    // //     };
    // //     let new_schedule = Schedule {
    // //         id: 999,
    // //         subject: "新規予定".to_string(),
    // //         start: naive_date_time(2024, 1, 1, 19, 0, 0),
    // //         end: naive_date_time(2024, 1, 1, 20, 0, 0),
    // //     };
    // //     assert!(schedule.intersects(&new_schedule));
    // // }
    // // #[test]
    // // fn test_schedule_intersects_4() {
    // //     let schedule = Schedule {
    // //         id: 1,
    // //         subject: "既存予定4".to_string(),
    // //         start: naive_date_time(2024, 1, 1, 20, 15, 0),
    // //         end: naive_date_time(2024, 1, 1, 20, 45, 0),
    // //     };
    // //     let new_schedule = Schedule {
    // //         id: 999,
    // //         subject: "新規予定".to_string(),
    // //         start: naive_date_time(2024, 1, 1, 19, 0, 0),
    // //         end: naive_date_time(2024, 1, 1, 20, 0, 0),
    // //     };
    // //     assert!(!schedule.intersects(&new_schedule));
    // // }
    // // #[test]
    // // fn test_schedule_intersects_5() {
    // //     let schedule = Schedule {
    // //         id: 1,
    // //         subject: "既存予定5".to_string(),
    // //         start: naive_date_time(2023, 12, 8, 9, 0, 0),
    // //         end: naive_date_time(2023, 12, 8, 10, 30, 0),
    // //     };
    // //     let new_schedule = Schedule {
    // //         id: 999,
    // //         subject: "新規予定".to_string(),
    // //         start: naive_date_time(2023, 12, 15, 10, 0, 0),
    // //         end: naive_date_time(2023, 12, 15, 11, 0, 0),
    // //     };
    // //     assert!(!schedule.intersects(&new_schedule));
    // // }
    // // #[test]
    // // fn test_schedule_intersects_6() {
    //     let schedule = Schedule {
    //         id: 1,
    //         subject: "既存予定6".to_string(),
    //         start: naive_date_time(2024, 1, 1, 19, 15, 0),
    //         end: naive_date_time(2024, 1, 1, 19, 45, 0),
    //     };
    //     let new_schedule = Schedule {
    //         id: 999,
    //         subject: "新規予定".to_string(),
    //         start: naive_date_time(2024, 1, 1, 19, 0, 0),
    //         end: naive_date_time(2024, 1, 1, 20, 0, 0),
    //     };
    //     assert!(schedule.intersects(&new_schedule));
    // }
}
