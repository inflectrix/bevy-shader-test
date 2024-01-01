mod custom_mat;

use custom_mat::CustomMaterial;

use bevy::{prelude::*, render::mesh::shape::Cube};

#[derive(Component)]
struct Spinny;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (spin, change_color))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 2., 0.).looking_at(Vec3 { x: 0., y: 0., z: -5. }, Vec3::Y),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 10000.,
            radius: 1.,
            ..default()
        },
        transform: Transform::from_xyz(0., 1., 0.),
        ..default()
    });

    // cube
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(Cube { size: 1. })),
            material: materials.add(CustomMaterial { time: 0., alpha_mode: AlphaMode::Blend }),
            transform: Transform::from_xyz(0., 0., -5.),
            ..default()
        },
        Spinny,
    ));
}

fn spin(
    time: Res<Time>,
    mut spinnies: Query<&mut Transform, With<Spinny>>,
) {
    for mut transform in &mut spinnies {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
    }
}

fn change_color(
    time: Res<Time>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds();
    }
}