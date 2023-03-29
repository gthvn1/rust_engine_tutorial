use rusty_engine::prelude::*;

struct GameState {
    //high_score: u32,
    current_score: u32,
    car_index: u32,
    //enemy_labels: Vec<String>,
    //spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            //high_score: 0,
            current_score: 0,
            car_index: 0,
            //enemy_labels: Vec::new(),
            //spawn_timer: Timer::from_seconds(1.0, false),
        }
    }
}

fn main() {
    let mut game = Game::new();
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);

    player.translation = Vec2::new(-400.0, 0.0);
    //player.rotation = std::f32::consts::FRAC_PI_2;
    player.scale = 0.5;
    player.collision = true;

    let car1 = game.add_sprite("uniqueCar", SpritePreset::RacingCarYellow);
    car1.translation = Vec2::new(300.0, 0.0);
    car1.scale = 0.5;
    car1.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

    // Handle collision
    //
    // An event looks like that:
    //     CollisionEvent {
    //         state: Begin,
    //         pair: CollisionPair(
    //             "car1",
    //             "player",
    //             ),
    //     }
    for event in engine.collision_events.drain(..) {
        println!("{:#?}", event);
        if event.state == CollisionState::Begin &&
            event.pair.one_starts_with("player") {
                // Check with whom we collision
                for label in [event.pair.0, event.pair.1] {
                    if label != "player" {
                        engine.sprites.remove(&label);
                    }
                }

                game_state.current_score += 1;
                println!("Current score: {}", game_state.current_score);
            }
    }

    // Handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    //player.translation.x += 100.0 * engine.delta_f32;
    const MOVEMENT_SPEED: f32 = 100.0;
    if engine.keyboard_state.pressed(KeyCode::Up) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.keyboard_state.pressed(KeyCode::Down) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.keyboard_state.pressed(KeyCode::Left) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.keyboard_state.pressed(KeyCode::Right) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    // Handle mouse event
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("car{}", game_state.car_index);
            // if the label already exists then this will move the sprite
            // to the mouse location. We want a new sprite so label must be
            // unique.
            game_state.car_index += 1;
            let car = engine.add_sprite(label, SpritePreset::RacingCarYellow);
            car.translation = mouse_location;
            car.scale = 0.5;
            car.collision = true;
        }
    }
}
