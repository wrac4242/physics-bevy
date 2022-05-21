use bevy::{prelude::*, render::camera::ScalingMode};
use bevy::window::PresentMode;

pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let height = 900.0;
    App::new()
        // resources
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Physics Simulator".to_string(),
            resizable: false,
            decorations: false,
            present_mode: PresentMode::Mailbox,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)

        // own systems
        .add_startup_system(spawn_camera)
        
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
