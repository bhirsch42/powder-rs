use super::*;

pub fn position_after_gravity(
    dust_particle_position: &DustParticlePosition,
    dust_world: &DustWorld,
) -> (usize, usize) {
    let down_one_row = dust_particle_position.row + 1;

    if down_one_row == dust_world.grid.len() {
        return (dust_particle_position.row, dust_particle_position.column);
    }

    let can_go_down = dust_world.grid[down_one_row][dust_particle_position.column].is_none();

    if can_go_down {
        return (down_one_row, dust_particle_position.column);
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
        let mut new_column = dust_particle_position.column + 1;

        if rand::random() {
            new_column = dust_particle_position.column - 1;
        }

        return (down_one_row, new_column);
    }

    if can_go_left {
        return (down_one_row, dust_particle_position.column - 1);
    }

    if can_go_right {
        return (down_one_row, dust_particle_position.column + 1);
    }

    (dust_particle_position.row, dust_particle_position.column)
}
