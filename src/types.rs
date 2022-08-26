
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    GameOver,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState::MainMenu
    }
}