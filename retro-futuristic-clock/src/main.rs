use bevy::prelude::*;
use chrono::{self, Timelike};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, clockface)
        .run();
}

fn setup(mut commands: Commands, mut gizmo_conf: ResMut<GizmoConfig>) {
    commands.spawn(Camera2dBundle::default());

    gizmo_conf.line_width = 20.0;
}

fn clockface(mut gizmos: Gizmos) {
    let now = chrono::Utc::now().naive_local();

    let hour = now.hour() as f32;
    let minute = now.minute() as f32;
    let second = now.second() as f32;

    let minute_angle = (360.0 / 60.0) * minute;
    let second_angle = (360.0 / 60.0) * second;
    let hour_angle = (360.0 / 24.0) * hour;

    // seconds
    gizmos.arc_2d(Vec2::ZERO, minute_angle.to_radians() / 2.0, minute_angle.to_radians(), 200., Color::BISQUE).segments(360*3);

    // minutes
    gizmos.arc_2d(Vec2::ZERO, second_angle.to_radians() / 2.0, second_angle.to_radians(), 250., Color::FUCHSIA).segments(360*3);

    // hour
    gizmos.arc_2d(Vec2::ZERO, hour_angle.to_radians() / 2.0, hour_angle.to_radians(), 300., Color::INDIGO).segments(360*3);
}
