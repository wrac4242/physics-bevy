use bevy::{prelude::*, render::camera::ScalingMode};
use bevy::window::PresentMode;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 900.0;

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
        .insert_resource(Gravity(Vec3::new(0.0, -1., 0.0)))

        .add_plugins(DefaultPlugins)

        // own systems
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_ball)

        // .add_system(apply_gravity)
        .add_system_set(
            SystemSet::new()
                .label("apply_physics")
                .with_system(apply_gravity)
                .with_system(constrain_area)
        )

        .add_system(update_positions.after("apply_physics"))
        
        // debugging
        .add_system(bevy::input::system::exit_on_esc_system)
        
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

#[derive(Component)]
struct BallProperties {
    prev_pos: Vec3,
    acceleration: Vec3,
    radius: f32,
}

impl BallProperties {
    pub fn accelerate(&mut self, acc: Vec3) {
        self.acceleration += acc;
    }
}


fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let radius = 100.0 / HEIGHT;
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("circle.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(radius , radius)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 10.),
            ..default()
    })
    .insert(BallProperties {
        prev_pos: Transform::from_xyz(0., 0., 10.).translation,
        acceleration: Vec3::new(0., 0., 0.),
        radius: radius,
    });
}

fn apply_gravity(gravity: Res<Gravity>, mut query: Query<&mut BallProperties>) {
    for mut ball in query.iter_mut() {
        ball.accelerate(gravity.0);
    }
}

fn constrain_area(mut query: Query<(&BallProperties, &mut Transform)>) {
    let constraint_pos = Vec3::new(100. / HEIGHT, 0., 0.);
    let radius = 300. / HEIGHT;

    for (ball, mut transform) in query.iter_mut() {
        let old_z = transform.translation.z;
        let mut current_pos = transform.translation;
        current_pos.z = 0.;
        let to_obj = current_pos - constraint_pos;
        let dist = to_obj.length();
        if dist > (radius - ball.radius) {
            let n = to_obj / dist;
            transform.translation = constraint_pos + n * (radius - ball.radius);
            transform.translation.z = old_z;
        }
    }
}


fn update_positions(time: Res<Time>, mut query: Query<(&mut BallProperties, &mut Transform)>) {
    for (mut properties, mut transform) in query.iter_mut() {
        let velocity = transform.translation - properties.prev_pos;
        properties.prev_pos = transform.translation;
        transform.translation = transform.translation + velocity + properties.acceleration * time.delta_seconds() * time.delta_seconds();

        properties.acceleration = Vec3::new(0., 0., 0.);
    }
}
