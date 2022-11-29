use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::Deserialize;
use bevy::reflect::TypeUuid;

pub use self::add_dust_world_texture::add_dust_world_texture;
use self::render_dust_world::render_dust_world;
use self::sync_particles_to_grids::sync_particles_to_grids;
use self::update_dust_particles::update_dust_particles;

mod add_dust_world_texture;
mod position_after_gravity;
mod render_dust_world;
mod sync_particles_to_grids;
mod update_dust_particles;

pub const DUST_WORLD_SIZE: usize = 512;
pub const PARALLEL_BATCH_SIZE: usize = 40000;

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
    pub prev_row: usize,
    pub prev_column: usize,
}

impl DustParticlePosition {
    pub fn new(position: (usize, usize)) -> Self {
        DustParticlePosition {
            row: position.0,
            column: position.1,
            prev_row: position.0,
            prev_column: position.1,
        }
    }

    pub fn update(&mut self, position: (usize, usize)) {
        self.prev_row = self.row;
        self.prev_column = self.column;
        self.row = position.0;
        self.column = position.1;
    }
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
        app.add_asset::<DustWorld>()
            .add_system(sync_particles_to_grids)
            .add_system(update_dust_particles.after(sync_particles_to_grids))
            .add_system(render_dust_world.after(sync_particles_to_grids));
    }
}
