use super::{DustParticle, DustParticlePosition, DustWorld, DUST_WORLD_SIZE};
use bevy::prelude::*;

pub fn clear_dust_world_grids(
    query: Query<&Handle<DustWorld>>,
    mut dust_worlds: ResMut<Assets<DustWorld>>,
) {
    for dust_world_handle in &query {
        let dust_world = dust_worlds.get_mut(dust_world_handle).unwrap();
        dust_world.grid.fill([None; DUST_WORLD_SIZE]);
    }
}

pub fn sync_particles_to_grids(
    query: Query<(&DustParticle, &DustParticlePosition, &Handle<DustWorld>)>,
    mut dust_worlds: ResMut<Assets<DustWorld>>,
) {
    for (dust_particle, dust_particle_position, dust_world_handle) in &query {
        let dust_world = dust_worlds.get_mut(dust_world_handle).unwrap();

        dust_world.grid[dust_particle_position.row][dust_particle_position.column] =
            Some(*dust_particle);
    }
}
