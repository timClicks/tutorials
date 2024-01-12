use bevy::prelude::*;
use bevy_inspector_egui::quick::*;
use bevy::pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin};
use noise::*;

use bevy_grid_mesh::*;

fn main() {
    let noise = Fbm::<Perlin>::new(0);
    let noisy = move |x: f32, y: f32| noise.get([x as f64, y as f64]) as f32 * 10. * (-0.001 * y);

    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default(),
            HeightMapPlugin::new(noisy, Color::DARK_GRAY, Vec2::ONE * 100.),
            WireframePlugin,

        ))
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Color::BLACK,
        })
        .add_systems(Startup, spawn)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(160., 160., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::X * 15. + Vec3::Y * 20.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        Name::new("Grid"),
        Grid::new(Vertex::new(100, 100), Vec2::ONE),
    ));
    commands.spawn((
        Name::new("Active Node"),
        GridTracker,
        SpatialBundle::default(),
    ));
}