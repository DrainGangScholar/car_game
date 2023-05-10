use rand::prelude::*;
use rusty_engine::prelude::*;
fn main() {
    let mut game = Game::new();

    //create sprite
    let player = game.add_sprite("player", SpritePreset::RacingCarBlack);
    player.translation.x = -500.0;
    player.layer = 10.0;
    player.collision = true;

    //background music
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.15);

    //road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    //create obstacles
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    //health text :DDD
    let health_message = game.add_text("health_message", "Health:5");
    health_message.translation = Vec2::new(550.0, 320.0);
    game.add_logic(game_logic);
    game.run(GameState::default());
}
struct GameState {
    health: u8,
    lost: bool,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            health: 10,
            lost: false,
        }
    }
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    //lose??
    if game_state.lost {
        return;
    }
    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }
    let player = engine.sprites.get_mut("player").unwrap();
    player.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player.rotation = direction * 0.15;
    if player.translation.y < -360.0 || player.translation.y > 360.0 {
        game_state.health = 0;
    }
    //MOVE ROAD OBJECT TO THE LEFT
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }
    let health_message = engine.texts.get_mut("health_message").unwrap();
    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains("player") || event.state.is_end() {
            continue;
        }
        if game_state.health > 0 {
            game_state.health -= 1;
            health_message.value = format!("Health: {}", game_state.health);
            engine.audio_manager.play_sfx(SfxPreset::Impact1, 0.35);
        }
    }
    if game_state.health == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game over", "Game Over!");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.4);
    };
}
