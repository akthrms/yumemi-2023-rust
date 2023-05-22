use std::{env, process};
use yumemi_2023_rust::{extract_game_result, read_entry_logs, read_play_logs, sort_play_logs};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("invalid argument number: expected 2, got {}", args.len());
        process::exit(1);
    }

    let entry_logs = match read_entry_logs(&args[0]) {
        Ok(entry_log) => entry_log,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
    let mut play_logs = match read_play_logs(&args[1]) {
        Ok(play_log) => play_log,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
    sort_play_logs(&mut play_logs);

    println!("rank,player_id,handle_name,score");
    for result in extract_game_result(play_logs, entry_logs) {
        println!("{}", result);
    }
}
