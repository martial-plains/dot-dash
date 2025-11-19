/// Defines the difficulty or unlock stage of an achievement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, uniffi::Enum)]
pub enum AchievementLevel {
    Bronze,
    Silver,
    Gold,
}

/// Represents a single unlockable milestone.
/// Exposed to FFI for displaying user progress and badges.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, uniffi::Record)]
pub struct Achievement {
    pub id: String,   // Unique ID (e.g., "KOCH_MASTER")
    pub name: String, // Display name
    pub description: String,
    pub unlocked_level: AchievementLevel, // Current level unlocked
    pub unlocked_at_ms: u64,              // Timestamp of last unlock
}
