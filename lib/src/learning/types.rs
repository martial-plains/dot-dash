use crate::morse::types::MorseCharacterSet;

/// Represents a single unit of practice or a flashcard item.
/// Exposed to FFI for managing quiz/flashcard state.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PracticeItem {
    pub morse_code: String, // The pattern (e.g., "01" for A)
    pub character: char,    // The character being tested (e.g., 'A')
    pub wpm_target: u32,    // The target Words Per Minute for this item
    pub success_count: u32, // For Spaced Repetition/Koch: how many times it was correct
    pub last_tested: u64,   // Timestamp of last test for Spaced Repetition
}

/// Represents the data collected from one completed learning session.
/// Exposed to FFI for saving to history and calculating achievements.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SessionResults {
    pub character_set: Vec<MorseCharacterSet>, // Sets used in the session
    pub start_time_ms: u64,                    // Session start time (milliseconds)
    pub duration_seconds: u32,
    pub total_items: u32,
    pub correct_count: u32,
    pub average_wpm: f64,
    pub code_speed_wpm: u32,     // Target speed for characters
    pub farnsworth_spacing: u32, // Spacing WPM (if applicable)
}
