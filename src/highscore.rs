use std::fs;
use std::path::Path;

const HIGHSCORE_FILE: &str = "highscore.txt";

pub struct HighScore {
    pub score: usize,
    pub is_new_record: bool,
}

impl HighScore {
    pub fn new() -> Self {
        let score = Self::load_high_score();
        Self {
            score,
            is_new_record: false,
        }
    }

    pub fn check_and_update(&mut self, current_score: usize) -> bool {
        if current_score > self.score {
            self.score = current_score;
            self.is_new_record = true;
            self.save_high_score();
            true // New high score!
        } else {
            false
        }
    }

    pub fn reset_new_record_flag(&mut self) {
        self.is_new_record = false;
    }

    fn load_high_score() -> usize {
        if Path::new(HIGHSCORE_FILE).exists() {
            if let Ok(contents) = fs::read_to_string(HIGHSCORE_FILE) {
                if let Ok(score) = contents.trim().parse::<usize>() {
                    return score;
                }
            }
        }
        0 // Default high score
    }

    fn save_high_score(&self) {
        if let Err(e) = fs::write(HIGHSCORE_FILE, self.score.to_string()) {
            eprintln!("Failed to save high score: {}", e);
        }
    }
}
