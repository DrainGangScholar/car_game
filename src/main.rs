use rusty_engine::prelude::*;
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}
fn main() {
    let mut game = Game::new();
    let game_state = GameState {
        high_score: 2560,
        current_score: 0,
        enemy_labels: Vec::new(),
        spawn_timer: Timer::from_seconds(10.0, false),
    };
    game.run(game_state);
}
