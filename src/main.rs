use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        MaterialPlugin::<CustomMaterial>::default(),
    ));

    app.add_systems(Startup, setup);
    app.add_systems(Update, (update_frame, flush_frame, update_outline));

    app.run();
}

#[derive(Component)]
struct CustomTextureAnima {
    pub timer: Timer,
    pub textures: Vec<Handle<Image>>,
    pub cur: usize,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let char_01: Handle<Image> = asset_server.load("character01.png");
    let char_02: Handle<Image> = asset_server.load("character02.png");
    let char_03: Handle<Image> = asset_server.load("character03.png");
    let char_04: Handle<Image> = asset_server.load("character04.png");

    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(10.0))),
            transform: Transform {
                rotation: Quat::from_rotation_x(PI / 2.0),
                ..default()
            },
            material: materials.add(CustomMaterial {
                color: Color::BLACK,
                char_texture: None,
                weapon_texture: Some(asset_server.load("weapon01.png")),
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        },
        CustomTextureAnima {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            textures: vec![char_01, char_02, char_03, char_04],
            cur: 0,
        },
    ));

    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            far: 4000.0,
            ..default()
        }
        .into(),
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 50.0)
            .looking_to(Vec3::NEG_Z, Vec3::Y)
            .with_scale(Vec3::new(0.02, 0.02, 1.0)),
        ..default()
    });
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    char_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    weapon_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

fn update_frame(time: Res<Time>, mut query: Query<&mut CustomTextureAnima>) {
    for mut anima in query.iter_mut() {
        anima.timer.tick(time.delta());
        if anima.timer.just_finished() {
            anima.cur += 1;
            if anima.cur >= anima.textures.len() {
                anima.cur = 0;
            }
        }
    }
}

fn flush_frame(
    mut materials: ResMut<Assets<CustomMaterial>>,
    query: Query<(&CustomTextureAnima, &Handle<CustomMaterial>)>,
) {
    for (anima, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.char_texture = Some(anima.textures[anima.cur].clone());
        }
    }
}

fn update_outline(
    keyboard_input: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    query: Query<&Handle<CustomMaterial>>,
) {
    for material_handle in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            if keyboard_input.pressed(KeyCode::Space) {
                material.color = Color::WHITE;
            } else {
                material.color = Color::BLACK;
            }
        }
    }
}
