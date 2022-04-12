use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use heron::prelude::*;

const WINDOW_SIZE: (f32, f32) = (800.0, 600.0);

const BALL_SPEED: f32 = 200.0;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Goal;

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
        .add_system(detect_collisions)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Bouncy ball
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::ZERO),
            GlobalTransform::default(),
        ))
        .insert(Ball)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(20.0, 20.0, 0.0),
            border_radius: None,
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(RotationConstraints::lock())
        .insert(Velocity::from_linear(Vec3::new(1.0, 2.0, 0.0).normalize() * BALL_SPEED));

    // Top wall
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(0.0, 300.0 - 10.0, 0.0)),
            GlobalTransform::default(),
        ))
        .insert(Wall)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(400.0, 10.0, 0.0),
            border_radius: None,
        })
        .insert(RigidBody::Static);

    // Bottom wall
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(0.0, -300.0 + 10.0, 0.0)),
            GlobalTransform::default(),
        ))
        .insert(Wall)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(400.0, 10.0, 0.0),
            border_radius: None,
        })
        .insert(RigidBody::Static);

    // Left goal zone
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(-400.0 - 20.0, 0.0, 0.0)),
            GlobalTransform::default(),
        ))
        .insert(Goal)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(20.0, 300.0, 0.0),
            border_radius: None,
        })
        .insert(RigidBody::Sensor);

    // Right goal zone
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::new(400.0 + 20.0, 0.0, 0.0)),
            GlobalTransform::default(),
        ))
        .insert(Goal)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(20.0, 300.0, 0.0),
            border_radius: None,
        })
        .insert(RigidBody::Sensor);
}

fn detect_collisions(
    mut events: EventReader<CollisionEvent>,
    mut ball_q: Query<&mut Velocity, With<Ball>>,
    wall_q: Query<&Wall>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                eprintln!("CollisionStart({:?}, {:?})", data1.rigid_body_entity(), data2.rigid_body_entity());
                let entity1 = data1.rigid_body_entity();
                let entity2 = data2.rigid_body_entity();

                if let (Ok(mut ball_velocity), Ok(_wall)) = (ball_q.get_mut(entity1), wall_q.get(entity2)) {
                    eprintln!("Bounce1! Normals: {:?}", data1.normals());
                    if let Some(normal) = data1.normals().get(0) {
                        // Bounce back in the direction of the first normal found.
                        if normal.x != 0.0 {
                            ball_velocity.linear.x *= -1.0;
                        } else if normal.y != 0.0 {
                            ball_velocity.linear.y *= -1.0;
                        }
                    }
                }

                if let (Ok(mut ball_velocity), Ok(_wall)) = (ball_q.get_mut(entity2), wall_q.get(entity1)) {
                    eprintln!("Bounce2! Normals: {:?}", data2.normals());
                    if let Some(normal) = data2.normals().get(0) {
                        // Bounce back in the direction of the first normal found.
                        if normal.x != 0.0 {
                            ball_velocity.linear.x *= -1.0;
                        } else if normal.y != 0.0 {
                            ball_velocity.linear.y *= -1.0;
                        }
                    }
                }
            }
            CollisionEvent::Stopped(data1, data2) => {
                eprintln!("CollisionStop({:?}, {:?})", data1.rigid_body_entity(), data2.rigid_body_entity());
            }
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
