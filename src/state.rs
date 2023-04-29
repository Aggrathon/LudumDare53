use bevy::prelude::*;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    #[default]
    MainMenu,
    Level0,
}

impl GameState {
    pub fn next_level(&self) -> Self {
        match self {
            GameState::MainMenu => GameState::Level0,
            GameState::Level0 => GameState::MainMenu,
        }
    }
}
