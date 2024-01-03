mod custom_mat;

use custom_mat::CustomMaterial;

use bevy::{prelude::*, render::mesh::shape::{Cube, UVSphere, Torus}, core_pipeline::bloom::BloomSettings};
use bevy_egui::{egui::{self, Ui}, EguiContexts, EguiPlugin};

#[derive(Resource, Debug)]
struct UIState {
    previous_model: String,
    selected_model: String,
}

impl Default for UIState {
    fn default() -> Self {
        let v = "Cube".to_string();
        Self { previous_model: v.clone(), selected_model: v }
    }
}

#[derive(Component)]
struct Spinny;

#[derive(Component)]
struct CamControlled;

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
            cam_controls,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0., 2., 0.).looking_at(
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: -5.,
                },
                Vec3::Y,
            ),
            ..default()
        },
        BloomSettings::NATURAL,
        CamControlled,
    ));

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
    spawn_cube(&mut commands, &mut materials, &mut meshes);
}

fn gen_shaded_shape(
    materials: &mut ResMut<Assets<CustomMaterial>>
) -> MaterialMeshBundle<CustomMaterial> {
    MaterialMeshBundle {
        material: materials.add(CustomMaterial {
            time: 0.,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::from_xyz(0., 0., -5.),
        ..default()
    }
}

fn spawn_cube(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<CustomMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(Cube { size: 1. })),
            ..gen_shaded_shape(materials)
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
    // stuff used to change models
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    spinnies: Query<Entity, With<Spinny>>,

    // ui stuff
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
) {
    egui::Window::new("Choose Model").show(contexts.ctx_mut(), |ui| {
        egui::ComboBox::from_label("Model")
            .selected_text(format!("{}", ui_state.selected_model))
            .show_ui(ui, |ui| {
                add_option(&mut ui_state, ui, "Cube");
                add_option(&mut ui_state, ui, "Pyramid");
                add_option(&mut ui_state, ui, "Sphere");
                add_option(&mut ui_state, ui, "Torus");
            });
    });

    if ui_state.previous_model != ui_state.selected_model {
        for ent in spinnies.iter() {
            commands.entity(ent).despawn();
        }

        match ui_state.selected_model.as_str() {
            "Cube" => spawn_cube(&mut commands, &mut materials, &mut meshes),
            "Pyramid" => {
                todo!();
            },
            "Sphere" => {
                commands.spawn((
                    MaterialMeshBundle {
                        mesh: meshes.add(Mesh::from(UVSphere {
                            radius: 1.,
                            sectors: 50,
                            stacks: 50,
                        })),
                        ..gen_shaded_shape(&mut materials)
                    },
                    Spinny,
                ));
            },
            "Torus" => {
                commands.spawn((
                    MaterialMeshBundle {
                        mesh: meshes.add(Mesh::from(Torus {
                            radius: 1.,
                            ring_radius: 0.5,
                            subdivisions_segments: 50,
                            subdivisions_sides: 50,
                        })),
                        ..gen_shaded_shape(&mut materials)
                    },
                    Spinny,
                ));
            }
            _ => panic!("invalid ui state: {:?}", ui_state),
        }

        ui_state.previous_model = ui_state.selected_model.clone();
    }
}

fn add_option(ui_state: &mut UIState, ui: &mut Ui, name: &str) {
    ui.selectable_value(&mut ui_state.selected_model, name.to_string(), name);
}

fn cam_controls(
    time: Res<Time>,
    mut cams: Query<&mut Transform, With<CamControlled>>,
    keys: Res<Input<KeyCode>>,
) {
    let dt = time.delta_seconds();

    if keys.any_pressed([KeyCode::Q, KeyCode::Down]) {
        for mut trans in cams.iter_mut() {
            trans.translation.y -= 4.*dt;
            trans.look_at(Vec3::new(0., 0., -5.), Vec3::Y);
        }
    }

    if keys.any_pressed([KeyCode::E, KeyCode::Up]) {
        for mut trans in cams.iter_mut() {
            trans.translation.y += 4.*dt;
            trans.look_at(Vec3::new(0., 0., -5.), Vec3::Y);
        }
    }
}