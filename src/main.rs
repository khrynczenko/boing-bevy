use bevy::prelude::*;

const VERTICAL_DISTACE: f32 = 320.0;

const RACKET_SPEED: f32 = 200.0;

struct Player;
struct Enemy;

trait Controls {
    fn get_up_keycode(&self) -> KeyCode;
    fn get_down_keycode(&self) -> KeyCode;
}

impl Controls for Player {
    fn get_up_keycode(&self) -> KeyCode {
        KeyCode::Up
    }
    fn get_down_keycode(&self) -> KeyCode {
        KeyCode::Down
    }
}

impl Controls for Enemy {
    fn get_up_keycode(&self) -> KeyCode {
        KeyCode::W
    }
    fn get_down_keycode(&self) -> KeyCode {
        KeyCode::S
    }
}

struct Racket {
    speed: f32,
}

struct Ball {
    velocity: Vec3,
}

struct Scoreboard {
    score: usize,
}

struct Position {
    x: f32,
    y: f32,
}

fn setup(mut commands: Commands) {
    // Add the game's entities to our world
    commands
        // cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
}

fn add_rackets(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let racket_texture = assets_server.load("assets/images/bat00.png").unwrap();
    let ball_texture = assets_server.load("assets/images/ball.png").unwrap();
    commands
        .spawn(SpriteComponents {
            material: materials.add(racket_texture.into()),
            ..Default::default()
        })
        .with(Player)
        .with(Racket {
            speed: RACKET_SPEED,
        })
        .with(Position {
            //x: 10f32,
            //y: WINDOW_HEIGHT_PX / 2f32,
            x: -VERTICAL_DISTACE + 10.0,
            y: 0f32,
        })
        .spawn(SpriteComponents {
            material: materials.add(racket_texture.into()),
            ..Default::default()
        })
        .with(Enemy)
        .with(Racket {
            speed: RACKET_SPEED,
        })
        .with(Position {
            //x: WINDOW_WIDTH_PX - 10f32,
            //y: - WINDOW_HEIGHT_PX / 2f32,
            x: VERTICAL_DISTACE - 10.0,
            y: 0f32,
        })
        .spawn(SpriteComponents {
            material: materials.add(ball_texture.into()),
            ..Default::default()
        })
        .with(Ball {
            velocity: Vec3::new(0.0, 0.0, 0.0),
        })
        .with(Position { x: 0.0, y: 0.0 });
}

fn position_player_racket(mut player_query: Query<(&Player, &Position, &mut Translation)>) {
    for (_, position, mut translation) in &mut player_query.iter() {
        *translation.0.x_mut() = position.x;
        *translation.0.y_mut() = position.y;
    }
}

fn position_enemy_racket(mut enemy_query: Query<(&Enemy, &Position, &mut Translation)>) {
    for (_, position, mut translation) in &mut enemy_query.iter() {
        *translation.0.x_mut() = dbg!(position.x);
        *translation.0.y_mut() = position.y;
    }
}

fn position_ball(mut ball_query: Query<(&Ball, &Position, &mut Translation)>) {
    for (_, position, mut translation) in &mut ball_query.iter() {
        *translation.0.x_mut() = position.x;
        *translation.0.y_mut() = position.y;
    }
}

fn process_input<T>(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&T, &Racket, &mut Position, &mut Translation)>,
) where
    T: Controls + Send + Sync + 'static,
{
    for (controllable, racket, _, mut translation) in &mut query.iter() {
        if keyboard_input.pressed(controllable.get_up_keycode()) {
            *translation.0.y_mut() += racket.speed * time.delta_seconds;
        }

        if keyboard_input.pressed(controllable.get_down_keycode()) {
            *translation.0.y_mut() -= racket.speed * time.delta_seconds;
        }
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_startup_system(add_rackets.system())
        .add_startup_system(position_player_racket.system())
        .add_startup_system(position_enemy_racket.system())
        .add_startup_stage("Position Entities")
        .add_startup_system_to_stage("Position Entities", position_player_racket.system())
        .add_startup_system_to_stage("Position Entities", position_enemy_racket.system())
        .add_startup_system_to_stage("Position Entities", position_ball.system())
        .add_system(process_input::<Player>.system())
        .add_system(process_input::<Enemy>.system())
        .run();
}
