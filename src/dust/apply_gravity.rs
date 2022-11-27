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

        let can_go_down = dust_world.grid[down_one_row][dust_particle_position.column].is_none();

        if can_go_down {
            dust_particle_position.row = down_one_row;
            continue;
        }

        let mut can_go_left = false;
        let mut can_go_right = false;

        let right_one_column = dust_particle_position.column + 1;
        if right_one_column < dust_world.grid.len() {
            can_go_right = dust_world.grid[down_one_row][right_one_column].is_none();
        }

        if dust_particle_position.column > 0 {
            let left_one_column = dust_particle_position.column - 1;

            if left_one_column > 0 {
                can_go_left = dust_world.grid[down_one_row][left_one_column].is_none();
            }
        }

        if can_go_left && can_go_right {
            if rand::random() {
                dust_particle_position.column -= 1;
            } else {
                dust_particle_position.column += 1;
            }

            dust_particle_position.row = down_one_row;
            continue;
        }

        if can_go_left {
            dust_particle_position.column -= 1;
            dust_particle_position.row = down_one_row;
            continue;
        }

        if can_go_right {
            dust_particle_position.column += 1;
            dust_particle_position.row = down_one_row;
            continue;
        }
    }
}
