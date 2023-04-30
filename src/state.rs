use bevy::prelude::*;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    #[default]
    MainMenu,
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    LevelTest,
}

impl GameState {
    pub fn next_level(&self) -> Self {
        match self {
            GameState::MainMenu => GameState::Level0,
            GameState::Level0 => GameState::Level1,
            GameState::Level1 => GameState::Level2,
            GameState::Level2 => GameState::Level3,
            GameState::Level3 => GameState::Level4,
            GameState::Level4 => GameState::Level5,
            GameState::Level5 => GameState::Level6,
            GameState::Level6 => GameState::Level7,
            GameState::Level7 => GameState::MainMenu,
            _ => GameState::MainMenu,
        }
    }
}
