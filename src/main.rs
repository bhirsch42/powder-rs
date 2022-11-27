use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;
use bevy::{prelude::*, window::close_on_esc};
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .add_system(animate_rotation)
        .add_system(animate_scale)
        .add_system(randomize_color)
        .run();
}

const IMAGE_SIZE: usize = 256;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

#[derive(Component)]
struct RandomizeColor;

fn make_sprite(images: &mut ResMut<Assets<Image>>, position: Vec3) -> SpriteBundle {
    let rgba_values = [0u8; 4];

    let mut texture = Image::new_fill(
        Extent3d {
            width: IMAGE_SIZE as u32,
            height: IMAGE_SIZE as u32,
            ..default()
        },
        TextureDimension::D2,
        &rgba_values,
        TextureFormat::Rgba8Unorm,
    );

    texture.sampler_descriptor = ImageSampler::nearest();

    let image_handle = images.add(texture);

    SpriteBundle {
        texture: image_handle,
        transform: Transform::from_translation(position),
        ..default()
    }
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..5 {
        for j in 0..3 {
            commands.spawn((
                make_sprite(
                    &mut images,
                    Vec3 {
                        x: ((i - 2) * IMAGE_SIZE as i32) as f32,
                        y: ((j - 1) * IMAGE_SIZE as i32) as f32,
                        z: 0.0,
                    },
                ),
                RandomizeColor,
                AnimateScale,
            ));
        }
    }
}

fn randomize_color(
    mut images: ResMut<Assets<Image>>,
    mut query: Query<&mut Handle<Image>, With<RandomizeColor>>,
) {
    let mut rng = thread_rng();
    let rgba_value_groups = vec![[0u8; 1024]; IMAGE_SIZE];

    let rgba_values = rgba_value_groups
        .into_iter()
        .flat_map(|mut rgba_value_group| {
            rng.fill(&mut rgba_value_group);
            rgba_value_group
        })
        .collect();

    let mut texture = Image::new(
        Extent3d {
            width: IMAGE_SIZE as u32,
            height: IMAGE_SIZE as u32,
            ..default()
        },
        TextureDimension::D2,
        rgba_values,
        TextureFormat::Rgba8Unorm,
    );

    texture.sampler_descriptor = ImageSampler::nearest();

    for image_handle in &mut query {
        let image = images.get_mut(&image_handle).unwrap();
        image.clone_from(&texture);
    }
}

fn animate_rotation(time: Res<Time>, mut query: Query<&mut Transform, With<AnimateRotation>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos());
    }
}

fn animate_scale(time: Res<Time>, mut query: Query<&mut Transform, With<AnimateScale>>) {
    let scale = time.elapsed_seconds().sin() + 1.;

    for mut transform in &mut query {
        transform.scale = Vec3::from_array([scale, scale, scale])
    }
}
