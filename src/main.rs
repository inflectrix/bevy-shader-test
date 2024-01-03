mod custom_mat;

use custom_mat::CustomMaterial;

use bevy::{prelude::*, render::mesh::shape::Cube};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Resource)]
struct UIState {
    selected_model: String,
}

impl Default for UIState {
    fn default() -> Self {
        Self { selected_model: "Cube".to_string() }
    }
}

#[derive(Component)]
struct Spinny;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<CustomMaterial>::default(),
            EguiPlugin,
        ))
        .init_resource::<UIState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            spin, 
            change_color,
            handle_ui,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 2., 0.).looking_at(
            Vec3 {
                x: 0.,
                y: 0.,
                z: -5.,
            },
            Vec3::Y,
        ),
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
            material: materials.add(CustomMaterial {
                time: 0.,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., -5.),
            ..default()
        },
        Spinny,
    ));
}

fn spin(time: Res<Time>, mut spinnies: Query<&mut Transform, With<Spinny>>) {
    for mut transform in &mut spinnies {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
    }
}

fn change_color(time: Res<Time>, mut materials: ResMut<Assets<CustomMaterial>>) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds();
    }
}

fn handle_ui(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
) {
    egui::Window::new("Choose Model").show(contexts.ctx_mut(), |ui| {
        egui::ComboBox::from_label("Model")
            .selected_text(format!("{}", ui_state.selected_model))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut ui_state.selected_model, "Cube".to_string(), "Cube");
                ui.selectable_value(&mut ui_state.selected_model, "Pyramid".to_string(), "Pyramid");
                ui.selectable_value(&mut ui_state.selected_model, "Sphere".to_string(), "Sphere");
            });
    });
}