use bevy::prelude::*;

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

fn position(pos: f32) -> f32 {
    pos * NODE_SIZE
}

struct SnakeHead();

impl SnakeHead {
    fn new() -> Self {
        Self {}
    }
}

struct Apple;
struct SnakeTick(Timer);
struct DirectionController(Direction);

fn camera_spawn_system(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}

fn add_snake(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    println!("Spawning snake...");
    let head_position: Vec3 = Vec3::new(
        position(5.0),
        position(5.0),
        0.0
    );

    commands
        .spawn(SpriteBundle {
            material: materials.add(ColorMaterial::color(Color::GREEN)),
            sprite: Sprite::new(Vec2::new(NODE_SIZE, NODE_SIZE)),
            ..Default::default()
        })
        .with(SnakeHead::new())
        .with(Transform::from_translation(head_position))
        .with(DirectionController(Direction::RIGHT));
}

fn apple_spawner_system(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, With<Apple>>
) {
    let number_of_apples = query.iter().count();
    
    if  number_of_apples > 0 {
        return;
    }

    println!("Spawning apple...");
    let next_position = Vec3::new(
        position(2.0), 
        position(2.0), 
        position(2.0)
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
            controller.0
        };
        
        if dir != controller.0.opposite() {
            controller.0 = dir;
        }
    }

}

fn snake_head_movement_system(
    time: Res<Time>,
    mut timer: ResMut<SnakeTick>,
    mut query: Query<(&mut Transform, &mut DirectionController), With<SnakeHead>>
) {
    
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    
    for (mut transform, direction_controller) in query.iter_mut() {
        let mut new_translation = transform.translation.clone();
        let direction =direction_controller.0;

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

fn direction_to_vec3(direction: Direction) -> Vec3 {
    match direction {
        Direction::UP => Vec3::new(0.0, position(1.0), 0.0),
        Direction::RIGHT => Vec3::new(position(1.0), 0.0, 0.0),
        Direction::DOWN => Vec3::new(0.0, position(-1.0), 0.0),
        Direction::LEFT => Vec3::new(position(-1.0), 0.0, 0.0),
    }
}

fn apple_eating_system(
    commands: &mut Commands,
    snake_query: Query<(&Transform, &DirectionController), With<SnakeHead>>,
    apple_query: Query<(Entity, &Transform), With<Apple>>
) {
    for (apple_entity, apple_transform) in apple_query.iter() {
        for (snake_head_transform, direction_controller) in snake_query.iter() {
            let direction = direction_controller.0;
            let direction_offset = direction_to_vec3(direction);
            let x_difference = snake_head_transform.translation.x + direction_offset.x - apple_transform.translation.x;
            let y_difference = snake_head_transform.translation.y + direction_offset.y - apple_transform.translation.y;

            if x_difference + y_difference == 0.0 {
                commands.despawn(apple_entity);
                // TODO: grow snake
                // TODO: increase score
            }
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_resource(SnakeTick(Timer::from_seconds(0.5, true)))
        .add_startup_system(add_snake.system())
        .add_startup_system(camera_spawn_system.system())
        .add_system(apple_spawner_system.system())
        .add_system(snake_controls_system.system())
        .add_system(snake_head_movement_system.system())
        .add_system(apple_eating_system.system())
        .run();
}