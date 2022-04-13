use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use heron::prelude::*;

const WINDOW_SIZE: (f32, f32) = (800.0, 600.0);

const BALL_SPEED: f32 = 200.0;
const BALL_SIZE: f32 = 40.0;
const WALL_DEPTH: f32 = 20.0;
const GOAL_DEPTH: f32 = 40.0;
const PADDLE_SIZE: (f32, f32) = (20.0, 100.0);
const PADDLE_SPEED: f32 = 300.0;

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    collision_shape: CollisionShape,
    rigid_body: RigidBody,
    rotation_constraints: RotationConstraints,
    velocity: Velocity,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl BallBundle {
    fn new(translation: Vec3, velocity: Vec3) -> Self {
        let collision_shape = CollisionShape::Cuboid {
            half_extends: Vec3::new(BALL_SIZE / 2.0, BALL_SIZE / 2.0, 0.0),
            border_radius: None,
        };
        Self {
            ball: Ball,
            collision_shape,
            rigid_body: RigidBody::KinematicVelocityBased,
            rotation_constraints: RotationConstraints::lock(),
            velocity: Velocity::from_linear(velocity),
            transform: Transform::from_translation(translation),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Goal;

#[derive(Component)]
struct Paddle;

struct GameState {
    left_paddle: Entity,
    right_paddle: Entity,
    left_goal: Entity,
    right_goal: Entity,
    left_score: u8,
    right_score: u8,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pong!".into(),
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(spawn)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(camera_control)
        .add_system(paddle_control)
        .add_system(ball_bounce)
        .add_system(check_scored)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Bouncy ball
    let ball_bundle = BallBundle::new(Vec3::ZERO, Vec3::new(-1.0, 2.0, 0.0).normalize() * BALL_SPEED);
    commands.spawn_bundle(ball_bundle);

    // Top wall
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(0.0, (WINDOW_SIZE.1 / 2.0) - (WALL_DEPTH / 2.0), 0.0)),
            GlobalTransform::default(),
        ))
        .insert(Wall)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(WINDOW_SIZE.0 / 2.0, WALL_DEPTH / 2.0, 0.0),
            border_radius: None,
        })
        .insert(RigidBody::Static);

    // Bottom wall
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(0.0, -(WINDOW_SIZE.1 / 2.0) + (WALL_DEPTH / 2.0), 0.0)),
            GlobalTransform::default(),
        ))
        .insert(Wall)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(WINDOW_SIZE.0 / 2.0, WALL_DEPTH / 2.0, 0.0),
            border_radius: None,
        })
        .insert(RigidBody::Static);

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
    let left_paddle = commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(-(WINDOW_SIZE.0 / 2.0) + 50.0, 0.0, 0.0)),
            GlobalTransform::default(),
        ))
        .insert(Paddle)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(PADDLE_SIZE.0 / 2.0, PADDLE_SIZE.1 / 2.0, 0.0),
            border_radius: None,
        })
        // TODO: Figure out how to make the paddles not be pushed by the ball.
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(Velocity::default())
        .id();

    // Right paddle
    let right_paddle = commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new((WINDOW_SIZE.0 / 2.0) - 50.0, 0.0, 0.0)),
            GlobalTransform::default(),
        ))
        .insert(Paddle)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(PADDLE_SIZE.0 / 2.0, PADDLE_SIZE.1 / 2.0, 0.0),
            border_radius: None,
        })
        // TODO: Figure out how to make the paddles not be pushed by the ball.
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(Velocity::default())
        .id();

    commands.insert_resource(GameState {
        left_paddle,
        right_paddle,
        left_goal,
        right_goal,
        left_score: 0,
        right_score: 0,
    });
}

fn ball_bounce(
    mut events: EventReader<CollisionEvent>,
    mut ball_q: Query<&mut Velocity, With<Ball>>,
    barrier_q: Query<(), Or<(With<Wall>, With<Paddle>)>>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                // eprintln!("CollisionStart({:?}, {:?})", data1.rigid_body_entity(), data2.rigid_body_entity());
                let mut try_bounce = |entity1, entity2, normals: &[Vec2]| {
                    if let (Ok(mut ball_velocity), Ok(_wall)) = (ball_q.get_mut(entity1), barrier_q.get(entity2)) {
                        eprintln!("Bounce! Normals: {:?}", normals);
                        if let Some(normal) = normals.get(0) {
                            // Bounce back in the direction of the first normal found.
                            if normal.x != 0.0 {
                                ball_velocity.linear.x *= -1.0;
                            } else if normal.y != 0.0 {
                                ball_velocity.linear.y *= -1.0;
                            }
                        }
                    }
                };

                let entity1 = data1.rigid_body_entity();
                let entity2 = data2.rigid_body_entity();

                try_bounce(entity1, entity2, data1.normals());
                try_bounce(entity2, entity1, data2.normals());
            }
            CollisionEvent::Stopped(_data1, _data2) => {
                // eprintln!("CollisionStop({:?}, {:?})", data1.rigid_body_entity(), data2.rigid_body_entity());
            }
        }
    }
}

fn check_scored(
    mut events: EventReader<CollisionEvent>,
    mut ball_q: Query<(&mut GlobalTransform, &mut Velocity), With<Ball>>,
    mut game_state: ResMut<GameState>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                let mut check_scored_internal = |entity1, entity2| {
                    if let (Ok((mut ball_transform, mut ball_velocity)), true) = (ball_q.get_mut(entity1), entity2 == game_state.left_goal) {
                        game_state.right_score += 1;
                        println!("Right Scored! {} - {}", game_state.left_score, game_state.right_score);

                        ball_transform.translation = Vec3::ZERO;
                        ball_velocity.linear = Vec3::new(1.0, 2.0, 0.0).normalize() * BALL_SPEED;
                    } else if let (Ok((mut ball_transform, mut ball_velocity)), true) = (ball_q.get_mut(entity1), entity2 == game_state.right_goal) {
                        game_state.left_score += 1;
                        println!("Left Scored! {} - {}", game_state.left_score, game_state.right_score);

                        ball_transform.translation = Vec3::ZERO;
                        ball_velocity.linear = Vec3::new(1.0, 2.0, 0.0).normalize() * BALL_SPEED;
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
    mut paddle_q: Query<&mut Velocity, With<Paddle>>,
    game_state: Res<GameState>,
) {
    if let Ok(mut paddle_velocity) = paddle_q.get_mut(game_state.left_paddle) {
        paddle_velocity.linear.y = 0.0;
        if keys.pressed(KeyCode::W) {
            paddle_velocity.linear.y += PADDLE_SPEED;
        }

        if keys.pressed(KeyCode::S) {
            paddle_velocity.linear.y -= PADDLE_SPEED;
        }
    }

    if let Ok(mut paddle_velocity) = paddle_q.get_mut(game_state.right_paddle) {
        paddle_velocity.linear.y = 0.0;
        if keys.pressed(KeyCode::Up) {
            paddle_velocity.linear.y += PADDLE_SPEED;
        }

        if keys.pressed(KeyCode::Down) {
            paddle_velocity.linear.y -= PADDLE_SPEED;
        }
    }
}
