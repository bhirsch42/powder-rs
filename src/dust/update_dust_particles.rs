use bevy::prelude::*;

use self::position_after_gravity::position_after_gravity;
use super::*;

pub fn update_dust_particles(
    mut query: Query<(&mut DustParticlePosition, &Handle<DustWorld>), With<DustParticleDynamic>>,
    dust_worlds: ResMut<Assets<DustWorld>>,
) {
    query.par_for_each_mut(
        PARALLEL_BATCH_SIZE,
        |(mut dust_particle_position, dust_world_handle)| {
            let dust_world = dust_worlds.get(dust_world_handle).unwrap();
            let new_position = position_after_gravity(&dust_particle_position, dust_world);
            dust_particle_position.update(new_position);
        },
    );
}
