use bevy::prelude::*;

use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;

use super::*;

pub fn render_dust_world(
    query: Query<(&mut Handle<Image>, &Handle<DustWorld>)>,
    mut images: ResMut<Assets<Image>>,
    dust_worlds: ResMut<Assets<DustWorld>>,
) {
    query.for_each(|(image_handle, dust_world)| {
        let image = images.get_mut(image_handle).unwrap();
        let dust_world = dust_worlds.get(dust_world).unwrap();

        let rgba_values: Vec<u8> = dust_world
            .grid
            .iter()
            .flat_map(|row| {
                row.iter().flat_map(|dust_particle| match dust_particle {
                    Some(particle) => particle.color(),
                    None => [0u8, 0u8, 0u8, 255u8],
                })
            })
            .collect();

        let mut texture = Image::new(
            Extent3d {
                width: DUST_WORLD_SIZE as u32,
                height: DUST_WORLD_SIZE as u32,
                ..default()
            },
            TextureDimension::D2,
            rgba_values,
            TextureFormat::Rgba8Unorm,
        );

        texture.sampler_descriptor = ImageSampler::nearest();
        image.clone_from(&texture);
    })
}
