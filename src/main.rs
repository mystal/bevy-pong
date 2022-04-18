#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::math::Mat2;
use heron::prelude::*;

const WINDOW_SIZE: (f32, f32) = (800.0, 600.0);

const BALL_SPEED: f32 = 300.0;
const BALL_SIZE: f32 = 40.0;
const WALL_SIZE:(f32, f32) = (WINDOW_SIZE.0, 20.0);
const GOAL_DEPTH: f32 = 40.0;
const PADDLE_SIZE: (f32, f32) = (20.0, 100.0);
const PADDLE_SPEED: f32 = 300.0;
const MAX_BOUNCE_ANGLE: f32 = 45.0;

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    #[bundle]
    sprite_bundle: SpriteBundle,
    collision_shape: CollisionShape,
    rigid_body: RigidBody,
    rotation_constraints: RotationConstraints,
    velocity: Velocity,
}

impl BallBundle {
    fn new(translation: Vec3, velocity: Vec3) -> Self {
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(translation),
            ..default()
        };
        let collision_shape = CollisionShape::Cuboid {
            half_extends: Vec3::new(BALL_SIZE / 2.0, BALL_SIZE / 2.0, 0.0),
            border_radius: None,
        };
        Self {
            ball: Ball,
            sprite_bundle,
            collision_shape,
            rigid_body: RigidBody::KinematicVelocityBased,
            rotation_constraints: RotationConstraints::lock(),
            velocity: Velocity::from_linear(velocity),
        }
    }

    fn from_side(side: PlayerSide) -> Self {
        let angle = (fastrand::f32() * MAX_BOUNCE_ANGLE * 2.0) - MAX_BOUNCE_ANGLE;
        let direction = Vec2::X * side.multiplier() as f32;
        let direction = Mat2::from_angle(angle.to_radians()).mul_vec2(direction);
        Self::new(Vec3::ZERO, direction.extend(0.0) * BALL_SPEED)
    }
}

#[derive(Component)]
struct Wall;

#[derive(Bundle)]
struct WallBundle {
    wall: Wall,
    #[bundle]
    sprite_bundle: SpriteBundle,
    collision_shape: CollisionShape,
    rigid_body: RigidBody,
    rotation_constraints: RotationConstraints,
}

impl WallBundle {
    fn new(translation: Vec3) -> Self {
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(WALL_SIZE.0, WALL_SIZE.1)),
                ..default()
            },
            transform: Transform::from_translation(translation),
            ..default()
        };
        let collision_shape = CollisionShape::Cuboid {
            half_extends: Vec3::new(WALL_SIZE.0 / 2.0, WALL_SIZE.1 / 2.0, 0.0),
            border_radius: None,
        };
        Self {
            wall: Wall,
            sprite_bundle,
            collision_shape,
            rigid_body: RigidBody::Static,
            rotation_constraints: RotationConstraints::lock(),
        }
    }
}

#[derive(Component)]
struct Goal;

#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    #[bundle]
    sprite_bundle: SpriteBundle,
    collision_shape: CollisionShape,
    rigid_body: RigidBody,
    rotation_constraints: RotationConstraints,
    velocity: Velocity,
    collisions: Collisions,
}

impl PaddleBundle {
    fn new(translation: Vec3) -> Self {
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PADDLE_SIZE.0, PADDLE_SIZE.1)),
                ..default()
            },
            transform: Transform::from_translation(translation),
            ..default()
        };
        let collision_shape = CollisionShape::Cuboid {
            half_extends: Vec3::new(PADDLE_SIZE.0 / 2.0, PADDLE_SIZE.1 / 2.0, 0.0),
            border_radius: None,
        };
        Self {
            paddle: Paddle,
            sprite_bundle,
            collision_shape,
            rigid_body: RigidBody::KinematicVelocityBased,
            rotation_constraints: RotationConstraints::lock(),
            velocity: Velocity::default(),
            collisions: Collisions::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PlayerSide {
    Left,
    Right,
}

const SIDES: &[PlayerSide] = &[PlayerSide::Left, PlayerSide::Right];

impl PlayerSide {
    fn random() -> Self {
        SIDES[fastrand::usize(..SIDES.len())]
    }

    fn next(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn multiplier(&self) -> i8 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}

struct PlayerScoredEvent(PlayerSide);

struct GameState {
    top_wall: Entity,
    bottom_wall: Entity,
    left_paddle: Entity,
    right_paddle: Entity,
    left_goal: Entity,
    right_goal: Entity,
    next_serve: PlayerSide,
    left_score: u8,
    right_score: u8,
}

fn main() {
    // When building for WASM, print panics to the browser console.
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pong!".into(),
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_event::<PlayerScoredEvent>()
        .add_startup_system(spawn)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(camera_control)
        .add_system(paddle_control)
        .add_system(ball_wall_bounce)
        .add_system(ball_paddle_bounce)
        .add_system(check_scored)
        .add_system(reset_round.after(check_scored))
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let initial_serve = PlayerSide::random();

    // Bouncy ball
    // let ball_bundle = BallBundle::new(Vec3::ZERO, Vec3::new(-1.0, 2.0, 0.0).normalize() * BALL_SPEED);
    let ball_bundle = BallBundle::from_side(initial_serve);
    commands.spawn_bundle(ball_bundle);

    // Top wall
    let wall_bundle = WallBundle::new(Vec3::new(0.0, (WINDOW_SIZE.1 / 2.0) - (WALL_SIZE.1 / 2.0), 0.0));
    let top_wall = commands
        .spawn_bundle(wall_bundle)
        .id();

    // Bottom wall
    let wall_bundle = WallBundle::new(Vec3::new(0.0, -(WINDOW_SIZE.1 / 2.0) + (WALL_SIZE.1 / 2.0), 0.0));
    let bottom_wall = commands
        .spawn_bundle(wall_bundle)
        .id();

    // Left goal zone
    let left_goal = commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(-(WINDOW_SIZE.0 / 2.0) - (GOAL_DEPTH / 2.0), 0.0, 0.0)),
            GlobalTransform::default(),
        ))
        .insert(Goal)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(GOAL_DEPTH / 2.0, WINDOW_SIZE.1 / 2.0, 0.0),
            border_radius: None,
        })
        .insert(RigidBody::Sensor)
        .id();

    // Right goal zone
    let right_goal = commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new((WINDOW_SIZE.0 / 2.0) + (GOAL_DEPTH / 2.0), 0.0, 0.0)),
            GlobalTransform::default(),
        ))
        .insert(Goal)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(GOAL_DEPTH / 2.0, WINDOW_SIZE.1 / 2.0, 0.0),
            border_radius: None,
        })
        .insert(RigidBody::Sensor)
        .id();

    // Left paddle
    let paddle_bundle = PaddleBundle::new(Vec3::new(-(WINDOW_SIZE.0 / 2.0) + 50.0, 0.0, 0.0));
    let left_paddle = commands.spawn_bundle(paddle_bundle).id();

    // Right paddle
    let paddle_bundle = PaddleBundle::new(Vec3::new((WINDOW_SIZE.0 / 2.0) - 50.0, 0.0, 0.0));
    let right_paddle = commands.spawn_bundle(paddle_bundle).id();

    commands.insert_resource(GameState {
        top_wall,
        bottom_wall,
        left_paddle,
        right_paddle,
        left_goal,
        right_goal,
        next_serve: initial_serve.next(),
        left_score: 0,
        right_score: 0,
    });
}

fn ball_wall_bounce(
    mut events: EventReader<CollisionEvent>,
    mut ball_q: Query<&mut Velocity, With<Ball>>,
    wall_q: Query<(), With<Wall>>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                let mut try_bounce = |entity1, entity2| {
                    if let (Ok(mut ball_velocity), Ok(_wall)) = (ball_q.get_mut(entity1), wall_q.get(entity2)) {
                        // The ball hit a wall, so simply reverse the y velocity.
                        ball_velocity.linear.y *= -1.0;
                    }
                };

                let entity1 = data1.rigid_body_entity();
                let entity2 = data2.rigid_body_entity();

                try_bounce(entity1, entity2);
                try_bounce(entity2, entity1);
            }
            _ => {}
        }
    }
}

fn ball_paddle_bounce(
    mut events: EventReader<CollisionEvent>,
    mut ball_q: Query<(&Transform, &mut Velocity), With<Ball>>,
    paddle_q: Query<&Transform, With<Paddle>>,
    game_state: Res<GameState>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                let mut try_bounce = |entity1, entity2| {
                    if let (Ok((ball_transform, mut ball_velocity)), Ok(paddle_transform)) = (ball_q.get_mut(entity1), paddle_q.get(entity2)) {
                        let multiplier = if entity2 == game_state.left_paddle { 1.0 } else { -1.0 };

                        // TODO: If the ball hit the top or bottom of a paddle, reflect the Y velocity.
                        // The ball hit a paddle. Figure out what new angle to come back at based where they collided.
                        let distance_from_center = ball_transform.translation.y - paddle_transform.translation.y;
                        let ratio_from_center = (distance_from_center / (PADDLE_SIZE.1 / 2.0)).clamp(-1.0, 1.0);
                        let bounce_angle = MAX_BOUNCE_ANGLE * ratio_from_center * multiplier;
                        let new_direction = Vec2::X * multiplier;
                        let new_direction = Mat2::from_angle(bounce_angle.to_radians()).mul_vec2(new_direction);
                        ball_velocity.linear = ball_velocity.linear.length() * new_direction.extend(0.0);
                    }
                };

                let entity1 = data1.rigid_body_entity();
                let entity2 = data2.rigid_body_entity();

                try_bounce(entity1, entity2);
                try_bounce(entity2, entity1);
            }
            _ => {}
        }
    }
}

fn check_scored(
    mut collisions: EventReader<CollisionEvent>,
    mut player_scored: EventWriter<PlayerScoredEvent>,
    ball_q: Query<(), With<Ball>>,
    game_state: Res<GameState>,
) {
    for event in collisions.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                let mut check_scored_internal = |entity1, entity2| {
                    if let (Ok(()), true) = (ball_q.get(entity1), entity2 == game_state.left_goal) {
                        player_scored.send(PlayerScoredEvent(PlayerSide::Right));
                    } else if let (Ok(()), true) = (ball_q.get(entity1), entity2 == game_state.right_goal) {
                        player_scored.send(PlayerScoredEvent(PlayerSide::Left));
                    }
                };

                let entity1 = data1.rigid_body_entity();
                let entity2 = data2.rigid_body_entity();

                check_scored_internal(entity1, entity2);
                check_scored_internal(entity2, entity1);
            }
            _ => {}
        }
    }
}

fn reset_round(
    mut commands: Commands,
    mut player_scored: EventReader<PlayerScoredEvent>,
    mut game_state: ResMut<GameState>,
    ball_q: Query<Entity, With<Ball>>,
) {
    let mut should_reset = false;
    for event in player_scored.iter() {
        should_reset = true;
        match event.0 {
            PlayerSide::Left => {
                game_state.left_score += 1;
                println!("Left Scored! {} - {}", game_state.left_score, game_state.right_score);
            }
            PlayerSide::Right => {
                game_state.right_score += 1;
                println!("Right Scored! {} - {}", game_state.left_score, game_state.right_score);
            }
        }
    }
    if should_reset {
        // TODO: Figure out if we can get teleporting working! Sounds like teleporting won't work for KinematicVelocityBased bodies...
        // For now, respawn the ball in the center.
        for ball_entity in ball_q.iter() {
            commands.entity(ball_entity).despawn();
            let ball_bundle = BallBundle::from_side(game_state.next_serve);
            commands.spawn_bundle(ball_bundle);
            game_state.next_serve = game_state.next_serve.next();
        }
    }
}

fn camera_control(
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_events: EventReader<MouseMotion>,
    mut camera_q: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = camera_q.single_mut();

    if keys.just_pressed(KeyCode::Key0) {
        camera_transform.translation.x = 0.0;
        camera_transform.translation.y = 0.0;
    }

    if buttons.pressed(MouseButton::Left) {
        let mouse_delta = {
            let mut delta = Vec2::ZERO;
            for motion in motion_events.iter() {
                delta.x -= motion.delta.x;
                delta.y += motion.delta.y;
            }
            delta.extend(0.0)
        };
        // Move the camera by how much the mouse moved.
        camera_transform.translation += mouse_delta;
    }
}

fn paddle_control(
    keys: Res<Input<KeyCode>>,
    mut paddle_q: Query<(&mut Velocity, &Collisions), With<Paddle>>,
    game_state: Res<GameState>,
) {
    if let Ok((mut paddle_velocity, collisions)) = paddle_q.get_mut(game_state.left_paddle) {
        paddle_velocity.linear.y = 0.0;

        // If pressing up and not colliding with the top wall.
        if keys.pressed(KeyCode::W) && !collisions.contains(&game_state.top_wall) {
            paddle_velocity.linear.y += PADDLE_SPEED;
        }

        // If pressing down and not colliding with the bottom wall.
        if keys.pressed(KeyCode::S) && !collisions.contains(&game_state.bottom_wall) {
            paddle_velocity.linear.y -= PADDLE_SPEED;
        }
    }

    if let Ok((mut paddle_velocity, collisions)) = paddle_q.get_mut(game_state.right_paddle) {
        paddle_velocity.linear.y = 0.0;

        // If pressing up and not colliding with the top wall.
        if keys.pressed(KeyCode::Up) && !collisions.contains(&game_state.top_wall) {
            paddle_velocity.linear.y += PADDLE_SPEED;
        }

        // If pressing down and not colliding with the bottom wall.
        if keys.pressed(KeyCode::Down) && !collisions.contains(&game_state.bottom_wall) {
            paddle_velocity.linear.y -= PADDLE_SPEED;
        }
    }
}
