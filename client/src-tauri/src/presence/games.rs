//! Games database for process name matching.

use serde::{Deserialize, Serialize};

/// A game entry in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEntry {
    /// Process names that match this game (case-insensitive).
    pub process_names: Vec<String>,
    /// Optional command-line arguments to match.
    #[serde(default)]
    pub match_args: Vec<String>,
    /// Display name of the game.
    pub name: String,
    /// Type of activity.
    #[serde(rename = "type")]
    pub activity_type: String,
}

/// Database of known games.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamesDatabase {
    pub games: Vec<GameEntry>,
}

impl GamesDatabase {
    /// Load the games database from embedded JSON.
    pub fn load() -> Self {
        let json = include_str!("../../resources/games.json");
        serde_json::from_str(json).expect("Invalid games.json")
    }

    /// Find a game by process name (case-insensitive).
    pub fn find_by_process(&self, process_name: &str) -> Option<&GameEntry> {
        let lower = process_name.to_lowercase();
        self.games
            .iter()
            .find(|g| g.process_names.iter().any(|p| p.to_lowercase() == lower))
    }
}

impl Default for GamesDatabase {
    fn default() -> Self {
        Self::load()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_games_database() {
        let db = GamesDatabase::load();
        assert!(!db.games.is_empty(), "Games database should not be empty");
    }

    #[test]
    fn test_find_minecraft() {
        let db = GamesDatabase::load();
        let game = db.find_by_process("minecraft.exe");
        assert!(game.is_some());
        assert_eq!(game.unwrap().name, "Minecraft");
    }

    #[test]
    fn test_find_case_insensitive() {
        let db = GamesDatabase::load();
        let game = db.find_by_process("MINECRAFT.EXE");
        assert!(game.is_some());
        assert_eq!(game.unwrap().name, "Minecraft");
    }

    #[test]
    fn test_find_unknown_returns_none() {
        let db = GamesDatabase::load();
        let game = db.find_by_process("unknown_game.exe");
        assert!(game.is_none());
    }
}
