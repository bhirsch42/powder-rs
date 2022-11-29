use super::*;

pub fn sync_particles_to_grids(
    query: Query<
        (&DustParticle, &DustParticlePosition, &Handle<DustWorld>),
        Changed<DustParticlePosition>,
    >,
    mut dust_worlds: ResMut<Assets<DustWorld>>,
) {
    query.for_each(
        |(dust_particle, dust_particle_position, dust_world_handle)| {
            let dust_world = dust_worlds.get_mut(dust_world_handle).unwrap();

            dust_world.grid[dust_particle_position.prev_row][dust_particle_position.prev_column] =
                None;

            dust_world.grid[dust_particle_position.row][dust_particle_position.column] =
                Some(*dust_particle);
        },
    )
}
