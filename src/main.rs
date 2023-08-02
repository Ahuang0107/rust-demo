use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    app.add_systems(Startup, setup_camera);

    app.run();
}

fn setup_camera(mut c: Commands) {
    c.spawn(Camera2dBundle::default());
}
