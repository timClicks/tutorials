use bevy::{prelude::*, math::vec2};
use chrono::Timelike;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, clock_face)
        .run()
}

fn setup(mut commands: Commands, mut gizmo_conf: ResMut<GizmoConfig>) {
    commands.spawn(Camera2dBundle::default());

    gizmo_conf.line_width = 20.0;
}

fn clock_face(mut gizmos: Gizmos) {
    let now = chrono::Local::now();

    let hour = now.hour() as f32;
    let minute = now.minute() as f32;
    let second = now.second() as f32;

    let second_angle = ((360.0 / 60.0) * second).to_radians();
    let minute_angle = ((360.0 / 60.0) * minute).to_radians();
    let hour_angle = ((360.0 / 24.0) * hour).to_radians();

    // seconds
    gizmos.arc_2d(Vec2::ZERO, second_angle / 2.0, second_angle, 100., Color::BISQUE)
    .segments(360*3);

    // minutes
    gizmos.arc_2d(Vec2::ZERO, minute_angle / 2.0, minute_angle, 120., Color::TEAL)
        .segments(360*3);

    // hours
    gizmos.arc_2d(Vec2::ZERO, hour_angle / 2.0, hour_angle, 140., Color::ORANGE)
        .segments(360*3);


    // println!("{hour}:{minute}:{second}");
}