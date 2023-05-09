use std::f32::consts::E;

use rusty_engine::prelude::*;
const BACKGROUND_LAYER: f32 = 0.0;
const CHARACTER_LAYER: f32 = 1.0;
const EFFECTS_LAYER: f32 = 2.0;
const UI_BOTTOM_LAYER: f32 = 3.0;
const UI_TOP_LAYER: f32 = 4.0;
fn main() {
    let mut game = Game::new();

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.1);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlack);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = UP;
    player.scale = 1.0;
    player.collision = true;

    let score = game.add_text("score", "Score:0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score:0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}
struct GameState {
    high_score: u32,
    score: u32,
    barrel_index: u32,
    // spawn_timer: Timer,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            barrel_index: 0,
            // spawn_timer: Timer::from_seconds(1.0, false),
        }
    }
}
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                    let score = engine.texts.get_mut("score").unwrap();
                    score.value = format!("Current Score: {}", game_state.score);
                    if game_state.score > game_state.high_score {
                        game_state.high_score = game_state.score;
                        let high_score = engine.texts.get_mut("high_score").unwrap();
                        high_score.value = format!("High Score: {}", game_state.score);
                    }
                    engine.audio_manager.play_sfx(SfxPreset::Minimize2, 0.2);
                }
            }
            game_state.score += 1;
            format!("Current score: {}", game_state.score);
        }
    }
    //handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 100.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }
    //handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("barrel{}", game_state.barrel_index);
            game_state.barrel_index += 1;
            let barrel = engine.add_sprite(label, SpritePreset::RacingBarrierRed); //label.clone()??
            barrel.translation = mouse_location;
            barrel.collision = true;
        }
    }
    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
    }
}
