use bevy::window::PresentMode;
use bevy::{prelude::*, render::camera::ScalingMode};

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 900.0;
const BALL_COUNT: usize = 250;
const CONSTRAINT_RADIUS: f32 = 600.;
const BALL_RADIUS: f32 = 10.0;

#[derive(Default)]
struct Gravity(Vec3);

fn main() {
    App::new()
        // resources
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Physics Simulator".to_string(),
            resizable: false,
            decorations: false,
            present_mode: PresentMode::Mailbox,
            ..Default::default()
        })
        .insert_resource(Gravity(Vec3::new(0.0, -1. * HEIGHT, 0.0)))
        .add_plugins(DefaultPlugins)
        // own systems
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_ball)
        // .add_system(apply_gravity)
        .add_system_set(
            SystemSet::new()
                .label("apply_physics")
                .with_system(apply_gravity),
        )
        .add_system(collisions.after("apply_physics").label("collisions"))
        .add_system(constrain_area.after("collisions").label("constraint_area"))
        .add_system(
            dampen_velocity
                .after("constraint_area")
                .label("dampen_velocity"),
        )
        .add_system(update_positions.after("dampen_velocity"))
        // debugging
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0 * HEIGHT;
    camera.orthographic_projection.bottom = -1.0 * HEIGHT;

    camera.orthographic_projection.left = -1.0 * RESOLUTION * HEIGHT;
    camera.orthographic_projection.right = 1.0 * RESOLUTION * HEIGHT;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

#[derive(Component)]
struct BallProperties {
    prev_pos: Vec3,
    acceleration: Vec3,
    radius: f32,
}

fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..BALL_COUNT {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("circle.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BALL_RADIUS * 2., BALL_RADIUS * 2.)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0. + i as f32, 0. + i as f32, 10.),
                ..default()
            })
            .insert(BallProperties {
                prev_pos: Transform::from_xyz(0., 0., 10.).translation,
                acceleration: Vec3::new(0., 0., 0.),
                radius: BALL_RADIUS,
            });
    }
}

fn apply_gravity(gravity: Res<Gravity>, mut query: Query<&mut BallProperties>) {
    for mut ball in query.iter_mut() {
        ball.acceleration += gravity.0;
    }
}

fn constrain_area(mut query: Query<(&BallProperties, &mut Transform)>) {
    let constraint_pos = Vec3::new(100., 0., 0.);

    for (ball, mut transform) in query.iter_mut() {
        let old_z = transform.translation.z;
        let mut current_pos = transform.translation;
        current_pos.z = 0.;
        let to_obj = current_pos - constraint_pos;
        let dist = to_obj.length();
        if dist > (CONSTRAINT_RADIUS - ball.radius) {
            let n = to_obj / dist;
            transform.translation = constraint_pos + n * (CONSTRAINT_RADIUS - ball.radius);
            transform.translation.z = old_z;
        }
    }
}

fn update_positions(time: Res<Time>, mut query: Query<(&mut BallProperties, &mut Transform)>) {
    for (mut properties, mut transform) in query.iter_mut() {
        let velocity = transform.translation - properties.prev_pos;
        properties.prev_pos = transform.translation;
        transform.translation = transform.translation
            + velocity
            + properties.acceleration * time.delta_seconds() * time.delta_seconds();

        properties.acceleration = Vec3::new(0., 0., 0.);
    }
}

fn collisions(mut query: Query<(Entity, &mut BallProperties, &mut Transform)>) {
    let mut objects = Vec::new();
    for (entity, properties, transform) in query.iter_mut() {
        objects.push((entity, properties, transform));
    }

    for ind1 in 0..objects.len() {
        for ind2 in 0..objects.len() {
            let mut do_col = false;
            let mut n = Vec3::new(0., 0., 0.);
            let mut delta = 0.;
            {
                let obj1 = &objects[ind1];
                let obj2 = &objects[ind2];
                if obj1.0.id() != obj2.0.id() {
                    let mut axis_of_collision = obj1.2.translation - obj2.2.translation;
                    axis_of_collision.z = 0.;
                    let dist = axis_of_collision.length();

                    if dist < obj1.1.radius + obj2.1.radius {
                        n = axis_of_collision / dist;
                        delta = obj1.1.radius + obj2.1.radius - dist;
                        do_col = true;
                        n.z = 0.;
                    }
                }
            }
            if do_col {
                objects[ind1].2.translation += n * delta * 0.5;
                objects[ind2].2.translation -= n * delta * 0.5;
            }
        }
    }
}

fn dampen_velocity(time: Res<Time>, mut query: Query<(&mut BallProperties, &Transform)>) {
    if time.delta_seconds() < f32::EPSILON {
        return;
    };
    for (mut properties, transform) in query.iter_mut() {
        let velocity = (transform.translation - properties.prev_pos) / time.delta_seconds();
        properties.acceleration += velocity * -0.5;
    }
}
