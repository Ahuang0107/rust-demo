use bevy::prelude::*;

#[cfg(feature = "debug")]
mod debug;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_systems(Startup, setup_camera);

    #[cfg(feature = "debug")]
    app.add_plugins(debug::DebugPlugin);

    app.run();
}

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            image: asset_server.load("icon.png"),
            ..default()
        },
        Transform {
            scale: Vec3::splat(5.0),
            ..default()
        },
    ));
}
