use bevy::prelude::*;

use super::*;

pub fn apply_gravity(
    mut query: Query<(&mut DustParticlePosition, &Handle<DustWorld>), With<DustParticleDynamic>>,
    dust_worlds: ResMut<Assets<DustWorld>>,
) {
    for (mut dust_particle_position, dust_world_handle) in &mut query {
        let dust_world = dust_worlds.get(dust_world_handle).unwrap();
        let down_one_row = dust_particle_position.row + 1;

        if down_one_row == dust_world.grid.len() {
            continue;
        }

        let neighbor_below = dust_world.grid[down_one_row][dust_particle_position.column];

        if neighbor_below.is_none() {
            dust_particle_position.row = down_one_row
        }
    }
}
