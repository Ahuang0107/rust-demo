use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(Material2dPlugin::<CustomMaterial>::default());

    app.add_systems(Startup, setup_camera);

    app.run();
}

fn setup_camera(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    c.spawn(Camera2dBundle::default());
    c.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(161.0, 94.0))))
            .into(),
        material: materials.add(CustomMaterial {
            color: Color::RED,
            base_texture: asset_server.load("drawer copy.png"),
            mask_texture: asset_server.load("mask copy.png"),
        }),
        // transform: Transform::from_scale(Vec3::splat(500.0)),
        ..Default::default()
    });
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    base_texture: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    mask_texture: Handle<Image>,
}
