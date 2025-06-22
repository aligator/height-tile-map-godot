use rand::prelude::*;

pub struct DiamondSquareGenerator;

impl DiamondSquareGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(
        &self,
        size: usize,
        roughness: f32,
        max_height: i32,
        rng: &mut StdRng,
    ) -> Vec<Vec<i32>> {
        let mut map = vec![vec![-1; size]; size];

        // Set corner values
        map[0][0] = rng.gen_range(0..=max_height);
        map[size - 1][0] = rng.gen_range(0..=max_height);
        map[0][size - 1] = rng.gen_range(0..=max_height);
        map[size - 1][size - 1] = rng.gen_range(0..=max_height);

        let mut side_length = size - 1;
        let mut current_roughness = roughness;

        while side_length >= 2 {
            let half_length = side_length / 2;

            // Diamond step
            for x in (0..size - 1).step_by(side_length) {
                for y in (0..size - 1).step_by(side_length) {
                    self.diamond_step(
                        &mut map,
                        x,
                        y,
                        half_length,
                        side_length,
                        current_roughness,
                        max_height,
                        rng,
                    );
                }
            }

            // Square step
            let mut col = 0;
            for x in (0..=size).step_by(half_length) {
                col += 1;
                if col % 2 == 1 {
                    for y in (half_length..size).step_by(side_length) {
                        self.square_step(
                            &mut map,
                            x % size,
                            y % size,
                            half_length,
                            size,
                            current_roughness,
                            max_height,
                            rng,
                        );
                    }
                } else {
                    for y in (0..size).step_by(side_length) {
                        self.square_step(
                            &mut map,
                            x % size,
                            y % size,
                            half_length,
                            size,
                            current_roughness,
                            max_height,
                            rng,
                        );
                    }
                }
            }

            current_roughness /= 2.0;
            side_length /= 2;
        }

        map
    }

    fn diamond_step(
        &self,
        map: &mut Vec<Vec<i32>>,
        x: usize,
        y: usize,
        reach: usize,
        size: usize,
        roughness: f32,
        max_height: i32,
        rng: &mut StdRng,
    ) {
        let top_left = map[x][y];
        let top_right = map[x + size][y];
        let bottom_left = map[x][y + size];
        let bottom_right = map[x + size][y + size];

        let mut avg = (top_left + top_right + bottom_left + bottom_right) as f32 / 4.0;
        avg += rng.gen_range(-roughness..=roughness);

        let center_x = x + reach;
        let center_y = y + reach;

        // Clamp to valid range
        let mut local_max_height = max_height
            .min(top_left + reach as i32)
            .min(top_right + reach as i32)
            .min(bottom_left + reach as i32)
            .min(bottom_right + reach as i32);

        let mut local_min_height = 0
            .max(top_left - reach as i32)
            .max(top_right - reach as i32)
            .max(bottom_left - reach as i32)
            .max(bottom_right - reach as i32);

        // Check boundaries
        if center_x >= reach * 2 {
            let pos = map[center_x - reach * 2][center_y];
            if pos != -1 {
                local_min_height = local_min_height.max(pos - reach as i32 * 2);
                local_max_height = local_max_height.min(pos + reach as i32 * 2);
            }
        }
        if center_x + reach * 2 < map.len() {
            let pos = map[center_x + reach * 2][center_y];
            if pos != -1 {
                local_min_height = local_min_height.max(pos - reach as i32 * 2);
                local_max_height = local_max_height.min(pos + reach as i32 * 2);
            }
        }
        if center_y >= reach * 2 {
            let pos = map[center_x][center_y - reach * 2];
            if pos != -1 {
                local_min_height = local_min_height.max(pos - reach as i32 * 2);
                local_max_height = local_max_height.min(pos + reach as i32 * 2);
            }
        }
        if center_y + reach * 2 < map.len() {
            let pos = map[center_x][center_y + reach * 2];
            if pos != -1 {
                local_min_height = local_min_height.max(pos - reach as i32 * 2);
                local_max_height = local_max_height.min(pos + reach as i32 * 2);
            }
        }

        // Ensure min <= max
        local_min_height = local_min_height.max(0);
        local_max_height = local_max_height.max(local_min_height);

        let final_height = avg.round() as i32;
        map[center_x][center_y] = final_height.clamp(local_min_height, local_max_height);
    }

    fn square_step(
        &self,
        map: &mut Vec<Vec<i32>>,
        x: usize,
        y: usize,
        reach: usize,
        size: usize,
        roughness: f32,
        max_height: i32,
        rng: &mut StdRng,
    ) {
        let mut count = 0;
        let mut avg = 0.0;
        let mut local_max_height = max_height;
        let mut local_min_height = 0;

        if x >= reach {
            avg += map[x - reach][y] as f32;
            count += 1;
            local_min_height = local_min_height.max(map[x - reach][y] - reach as i32);
            local_max_height = local_max_height.min(map[x - reach][y] + reach as i32);
        }
        if x + reach < size {
            avg += map[x + reach][y] as f32;
            count += 1;
            local_min_height = local_min_height.max(map[x + reach][y] - reach as i32);
            local_max_height = local_max_height.min(map[x + reach][y] + reach as i32);
        }
        if y >= reach {
            avg += map[x][y - reach] as f32;
            count += 1;
            local_min_height = local_min_height.max(map[x][y - reach] - reach as i32);
            local_max_height = local_max_height.min(map[x][y - reach] + reach as i32);
        }
        if y + reach < size {
            avg += map[x][y + reach] as f32;
            count += 1;
            local_min_height = local_min_height.max(map[x][y + reach] - reach as i32);
            local_max_height = local_max_height.min(map[x][y + reach] + reach as i32);
        }

        avg /= count as f32;
        avg += rng.gen_range(-roughness..=roughness);

        // Ensure min <= max
        local_min_height = local_min_height.max(0);
        local_max_height = local_max_height.max(local_min_height);

        let final_height = avg.round() as i32;
        map[x][y] = final_height.clamp(local_min_height, local_max_height);
    }
}
