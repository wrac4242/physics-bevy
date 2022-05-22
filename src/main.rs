use bevy::{prelude::*, render::camera::ScalingMode};
use bevy::window::PresentMode;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 900.0;

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

        .add_plugins(DefaultPlugins)

        // own systems
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_ball)

        // .add_system(apply_gravity)
        .add_system_set(
            SystemSet::new()
                .label("apply_physics")
                .with_system(apply_gravity)
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
    prev_pos: Transform,
    acceleration: Vec2,
    radius: f32,
}

fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let radius = 100.0;
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("circle.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(1./HEIGHT * radius, 1./HEIGHT * radius)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 10.),
            ..default()
    })
    .insert(BallProperties {
        prev_pos: Transform::from_xyz(0., 0., 10.),
        acceleration: Vec2::new(0., 0.),
        radius: radius,
    });
}

fn apply_gravity() {
    todo!();
}

fn update_positions() {
    todo!();
}
