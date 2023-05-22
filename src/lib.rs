use serde::Deserialize;
use std::{cmp::Ordering, collections::HashMap, fmt::Display};

const MAX_RANK_CNT: u64 = 10;

#[derive(Debug, Deserialize, Clone)]
pub struct EntryLog {
    player_id: String,
    handle_name: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Clone)]
pub struct PlayLog {
    create_timestamp: String,
    player_id: String,
    score: u64,
}

#[derive(Debug, Clone)]
pub struct GameResult {
    rank: u64,
    player_id: String,
    handle_name: String,
    score: u64,
}

impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.rank, self.player_id, self.handle_name, self.score
        )
    }
}

pub fn read_entry_logs(filepath: &str) -> anyhow::Result<HashMap<String, EntryLog>> {
    let mut entry_logs = HashMap::new();

    let mut reader = csv::Reader::from_path(filepath)?;
    for entry_log in reader.deserialize() {
        let entry_log: EntryLog = entry_log?;
        entry_logs.insert(entry_log.player_id.clone(), entry_log);
    }

    Ok(entry_logs)
}

pub fn read_play_logs(filepath: &str) -> anyhow::Result<Vec<PlayLog>> {
    let mut play_logs = HashMap::new();

    let mut reader = csv::Reader::from_path(filepath)?;
    for play_log in reader.deserialize() {
        let play_log: PlayLog = play_log?;
        play_logs
            .entry(play_log.player_id.clone())
            .and_modify(|existing: &mut PlayLog| {
                if existing.score < play_log.score {
                    *existing = play_log.clone();
                }
            })
            .or_insert_with(|| play_log);
    }

    Ok(play_logs
        .into_iter()
        .map(|(_, play_log)| play_log)
        .collect())
}

pub fn sort_play_logs(play_logs: &mut [PlayLog]) {
    play_logs.sort_by(|a, b| match b.score.cmp(&a.score) {
        Ordering::Equal => a.player_id.cmp(&b.player_id),
        other => other,
    });
}

pub fn extract_game_result(
    play_logs: Vec<PlayLog>,
    entry_logs: HashMap<String, EntryLog>,
) -> Vec<GameResult> {
    let mut result = Vec::new();

    let mut rank = 1;
    let mut result_cnt = 1;
    let mut prev_score = 0;

    for play_log in play_logs {
        if let Some(entry_log) = entry_logs.get(&play_log.player_id) {
            if play_log.score < prev_score {
                rank = result_cnt;
            }

            if MAX_RANK_CNT < rank {
                break;
            }

            result.push(GameResult {
                rank,
                player_id: play_log.player_id,
                handle_name: entry_log.handle_name.clone(),
                score: play_log.score,
            });
            result_cnt += 1;
            prev_score = play_log.score;
        }
    }

    result
}
