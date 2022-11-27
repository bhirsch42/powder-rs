use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::Deserialize;
use bevy::reflect::TypeUuid;

pub use self::add_dust_world_texture::add_dust_world_texture;
use self::apply_gravity::apply_gravity;
use self::render_dust_world::render_dust_world;
use self::sync_particles_to_grids::{clear_dust_world_grids, sync_particles_to_grids};

mod add_dust_world_texture;
mod apply_gravity;
mod render_dust_world;
mod sync_particles_to_grids;

pub const DUST_WORLD_SIZE: usize = 128;

#[derive(Clone, Copy, Deserialize, Debug)]
pub enum DustParticleType {
    Powder,
}

#[derive(Component, Clone, Copy, Deserialize, Debug)]
pub struct DustParticle {
    pub dust_particle_type: DustParticleType,
}

impl DustParticle {
    fn color(self) -> [u8; 4] {
        match self.dust_particle_type {
            DustParticleType::Powder => [255u8, 0u8, 0u8, 255u8],
        }
    }
}

#[derive(Component)]
pub struct DustParticlePosition {
    pub row: usize,
    pub column: usize,
}

#[derive(Component)]
pub struct DustParticleDynamic;

#[derive(TypeUuid)]
#[uuid = "2b2ab282-74dc-4de7-a663-5384861a2699"]
pub struct DustWorld {
    grid: [[Option<DustParticle>; DUST_WORLD_SIZE]; DUST_WORLD_SIZE],
}

impl Default for DustWorld {
    fn default() -> Self {
        Self {
            grid: [[None; DUST_WORLD_SIZE]; DUST_WORLD_SIZE],
        }
    }
}

pub struct DustPlugin;

impl Plugin for DustPlugin {
    fn build(&self, app: &mut App) {
        let update_dust_world = apply_gravity;
        let buffer_dust_world = clear_dust_world_grids.pipe(sync_particles_to_grids);
        let run_dust_world = update_dust_world
            .pipe(buffer_dust_world)
            .pipe(render_dust_world);

        app.add_asset::<DustWorld>().add_system(run_dust_world);
    }
}
