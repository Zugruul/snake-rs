use bevy::prelude::*;
use rand::{thread_rng, Rng};

mod plugins;
use crate:: plugins::*;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::UP => Self::DOWN,
            Self::RIGHT => Self::LEFT,
            Self::DOWN => Self::UP,
            Self::LEFT => Self::RIGHT,
        }
    }
}

const NODE_SIZE: f32 = 20.0;
const PLAY_AREA_WIDTH: f32 = 13.0 * NODE_SIZE;
const PLAY_AREA_HEIGHT: f32 = 13.0 * NODE_SIZE;

fn position(x: i32, y: i32) -> Vec3 {
    Vec3::new(x as f32 * NODE_SIZE, y as f32 * NODE_SIZE, 0.0)
}

#[derive(Default)]
struct SnakeSegment(Vec<Entity>);

#[derive(Default)]
struct SnakeHead();

struct Apple;
struct GameTick(Timer);
struct Scoreboard(i32);
struct DirectionController {
    direction: Direction
}

impl DirectionController {
    fn new(direction: Direction) -> Self {
        Self {
            direction,
        }
    }
}

fn camera_spawn_system(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}

fn add_snake(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    println!("Spawning snake...");
    let head_position: Vec3 = position(0, 0);

    commands
        .spawn(SpriteBundle {
            material: materials.add(ColorMaterial::color(Color::GREEN)),
            sprite: Sprite::new(Vec2::new(NODE_SIZE, NODE_SIZE)),
            ..Default::default()
        })
        .with(SnakeHead::default())
        .with(Transform::from_translation(head_position))
        .with(DirectionController::new(Direction::RIGHT));
}

fn apple_spawner_system(
    commands: &mut Commands,
    timer: Res<GameTick>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, With<Apple>>
) {
    if !timer.0.just_finished() {
        return;
    }

    let number_of_apples = query.iter().count();
    
    if  number_of_apples > 0 {
        return;
    }

    let mut rng = thread_rng();

    let x = rng.gen_range(-0.5..0.5) * PLAY_AREA_WIDTH;
    let y = rng.gen_range(-0.5..0.5) * PLAY_AREA_HEIGHT;
    
    let x = x - (x % NODE_SIZE);
    let y = y - (y % NODE_SIZE);

    println!("Spawning apple at x={} y={}", &x, &y);
    let next_position = Vec3::new(
        x,
        y,
        0.0
    );
    commands.spawn(SpriteBundle {
        material: materials.add(ColorMaterial::color(Color::RED)),
        sprite: Sprite::new(Vec2::new(NODE_SIZE, NODE_SIZE)),
        ..Default::default()
    })
    .with(Apple)
    .with(Transform::from_translation(next_position));
}

fn snake_controls_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut DirectionController, With<SnakeHead>>
) {
    for mut controller in query.iter_mut() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::LEFT
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::DOWN
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::UP
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::RIGHT
        } else {
            controller.direction
        };
        
        if dir != controller.direction.opposite() {
            controller.direction = dir;
        }
    }

}

fn snake_head_movement_system(
    timer: Res<GameTick>,
    mut query: Query<(&mut Transform, &mut DirectionController), With<SnakeHead>>
) {
    
    if !timer.0.just_finished() {
        return;
    }
    
    for (mut transform, direction_controller) in query.iter_mut() {
        let mut new_translation = transform.translation.clone();
        let direction = direction_controller.direction;

        match direction {
            Direction::UP => new_translation.y += NODE_SIZE,
            Direction::RIGHT => new_translation.x += NODE_SIZE,
            Direction::DOWN => new_translation.y -= NODE_SIZE,
            Direction::LEFT => new_translation.x -= NODE_SIZE,
        }

        transform.translation = new_translation;
        println!("Snake at x={} y={}", transform.translation.x, transform.translation.y);
    }
}

fn apple_eating_system(
    commands: &mut Commands,
    mut score: ResMut<Scoreboard>,
    timer: Res<GameTick>,
    snake_query: Query<&Transform, With<SnakeHead>>,
    apple_query: Query<(Entity, &Transform), With<Apple>>
) {
    if !timer.0.just_finished() {
        return;
    }

    for (apple_entity, apple_transform) in apple_query.iter() {
        for snake_head_transform in snake_query.iter() {
            let same_x = snake_head_transform.translation.x == apple_transform.translation.x;
            let same_y = snake_head_transform.translation.y == apple_transform.translation.y;

            if same_x && same_y {
                println!("Ate apple at x={} y={}", &apple_transform.translation.x, &apple_transform.translation.y);
                commands.despawn(apple_entity);

                score.0 += 1;
                println!("Score: {}", &score.0);
                // TODO: grow snake
                // TODO: increase score
            }
        }
    }
}

fn timer_tick_system(time: Res<Time>, mut timer: ResMut<GameTick>) {
    timer.0.tick(time.delta_seconds());
}

fn add_scoreboard(
    commands: &mut Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(TextBundle {
        text: Text {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            value: format!("Score: "),
            style: TextStyle {
                color: Color::rgb(0.5, 0.8, 0.5),
                font_size: 20.0,
                ..Default::default()
            },
        },
        style: Style {
            position_type: PositionType::Relative,
            position: Rect {
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn scoreboard_system(
    score: Res<Scoreboard>,
    mut query: Query<&mut Text>
) {
    for mut text in query.iter_mut() {
        text.value = format!("Score: {}", score.0);
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: PLAY_AREA_WIDTH,
            height: PLAY_AREA_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_resource(Scoreboard(0))
        .add_resource(GameTick(Timer::from_seconds(0.5, true)))
        .add_startup_system(camera_spawn_system.system())
        .add_startup_system(add_scoreboard.system())
        .add_startup_system(add_snake.system())
        .add_system(timer_tick_system.system())
        .add_system(scoreboard_system.system())
        .add_system(apple_spawner_system.system())
        .add_system(snake_controls_system.system())
        .add_system(snake_head_movement_system.system())
        .add_system(apple_eating_system.system())
        .run();
}