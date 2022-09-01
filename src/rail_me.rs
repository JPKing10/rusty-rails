use crate::consts::*;
use bevy::{
    prelude::*,
    winit::WinitSettings
};

pub struct Level1;

impl Plugin for Level1 {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(system))
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(enter))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(exit));
    }
}

fn enter(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<StandardMaterial>>) {
    info!("Enter game state");

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.75, 0.0),
        ..default()
    });
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 2.5)
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
    });
}

fn system(time: Res<Time>, mut transforms: Query<&mut Transform, With<Camera3d>>) {
    for mut transform in transforms.iter_mut() {
        transform.rotate_y(0.001);
    }
}

fn exit() {}
