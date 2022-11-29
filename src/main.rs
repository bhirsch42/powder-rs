use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::close_on_esc};
use dust::{
    add_dust_world_texture, DustParticle, DustParticleDynamic, DustParticlePosition,
    DustParticleType, DustPlugin, DustWorld, DUST_WORLD_SIZE,
};

mod dust;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DustPlugin)
        .add_startup_system(setup)
        .add_startup_system(resize_window)
        .add_system(close_on_esc)
        .run();
}

fn resize_window(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    println!("Window size was: {},{}", window.width(), window.height());
    window.set_resolution(1280.0, 1000.0);
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut dust_worlds: ResMut<Assets<DustWorld>>,
) {
    commands.spawn(Camera2dBundle::default());

    let dust_world = dust_worlds.add(DustWorld::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3 {
                x: 1.75,
                y: 1.75,
                z: 1.75,
            }),
            texture: add_dust_world_texture(&mut images),
            ..default()
        },
        dust_world.clone(),
    ));

    for i in 0..500 {
        for j in 0..500 {
            commands.spawn((
                DustParticle {
                    dust_particle_type: DustParticleType::Powder,
                },
                DustParticleDynamic,
                DustParticlePosition::new((i, j)),
                dust_world.clone(),
            ));
        }
    }
}
