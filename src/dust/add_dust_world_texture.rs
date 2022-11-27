use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;

use super::DUST_WORLD_SIZE;

pub fn add_dust_world_texture(images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
    let rgba_values = [0u8; 4];

    let mut texture = Image::new_fill(
        Extent3d {
            width: DUST_WORLD_SIZE as u32,
            height: DUST_WORLD_SIZE as u32,
            ..default()
        },
        TextureDimension::D2,
        &rgba_values,
        TextureFormat::Rgba8Unorm,
    );

    texture.sampler_descriptor = ImageSampler::nearest();

    images.add(texture)
}
