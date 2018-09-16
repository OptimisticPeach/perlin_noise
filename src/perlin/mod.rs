pub mod interpolators;
mod random_2d;
//mod interpolators;
extern crate rand;
use self::interpolators::*;
use self::random_2d::{Access2dPercent, Randomizer2D};
use rand::Rng;
use std::{cmp, f32, usize};

pub fn fill_rand(size: usize) -> Vec<f32> {
    let mut randoms: Vec<f32> = Vec::new();

    for _ in 0..size {
        randoms.push(rand::thread_rng().gen_range(0.0, 1.0));
    }

    randoms
}

pub fn get_perlin(size: usize, depth: Option<usize>) -> Vec<f32> {
    let depth = depth.unwrap_or(usize::MAX);
    let randoms = fill_rand(size);
    let mut perlin: Vec<f32> = vec![0.0; size];

    let size_log2 = (size as f32).log2();

    let depth = if depth < size_log2.floor() as usize {
        depth
    } else {
        size_log2.floor() as usize
    };

    let depth = depth as usize;

    let mut recipr = 0;

    let mut avg_factor = 0.0;

    for depth_index in (0..depth).rev() {
        let power = (depth_index as f32).exp2() as usize;
        let power_recipr = 1.0 / (recipr as f32).exp2();
        avg_factor += power_recipr;

        recipr += 1;

        let mut rand_indexes: Vec<usize> = Vec::new();

        let rand_indexes_size = (size as usize / power) as usize;

        for j in 0..rand_indexes_size {
            rand_indexes.push(j * power);
        }

        rand_indexes.push(size as usize - 1);

        if rand_indexes_size == 0 {
            continue;
        }

        for current_rand_index in 0..(rand_indexes_size) {
            let prev_rand_index = rand_indexes[current_rand_index as usize] as usize;
            let next_rand_index = rand_indexes[(current_rand_index + 1) as usize] as usize;
            for i in prev_rand_index..next_rand_index {
                perlin[i as usize] += power_recipr * lerp(
                    randoms[prev_rand_index as usize],
                    randoms[next_rand_index as usize],
                    ((i - prev_rand_index) as f32) / ((next_rand_index - prev_rand_index) as f32),
                );
            }
        }
    }

    for i in 0..(size - 1) {
        perlin[i] /= avg_factor;
    }

    perlin[0..(size - 1)].to_vec()
}

pub fn fill_rand_2d(sizex: usize, sizey: usize) -> Vec<Vec<f32>> {
    let mut randoms: Vec<Vec<f32>> = Vec::new();
    for _ in 0..sizex {
        let mut x_line: Vec<f32> = Vec::new();
        for _ in 0..sizey {
            x_line.push(rand::thread_rng().gen_range(0.0, 1.0));
        }
        randoms.push(x_line);
    }

    randoms
}

pub fn get_perlin_2d<T: Fn() -> f32>(
    sizex: usize,
    sizey: usize,
    depth: Option<usize>,
    detail: Option<bool>,
    f: T
) -> Vec<Vec<f32>> {
    let size = cmp::max(sizex + 1, sizey + 1);
    let detail = detail.unwrap_or(false);

    let x_larger = cmp::max(sizex, sizey) == sizex;

    let depth = depth.unwrap_or(usize::MAX);
    let mut randoms = Randomizer2D::new(size, size, f);
    let mut perlin: Vec<Vec<f32>> = vec![vec![0.0; size]; size];

    let size_log2 = (size as f32).log2();

    let max_depth = size_log2.ceil() as usize;
    let depth = if depth < max_depth { depth } else { max_depth };

    let depth_power_list = (0..=depth).map(|x| {
        (
            ((max_depth - x) as f32).exp2() as usize,
            1.0 / (x as f32).exp2(),
        )
    });

    let mut avg_factor = 0.0;

    for (power, power_recipr) in depth_power_list {
        avg_factor += power_recipr;

        let mut rand_indexes_x: Vec<usize> = Vec::new();

        let rand_indexes_size_x = (size as f32 / power as f32) as usize;

        for j in 0..rand_indexes_size_x {
            rand_indexes_x.push(j * power);
        }

        rand_indexes_x.push(size as usize - 1);

        if rand_indexes_size_x == 0 {
            continue;
        }

        for current_rand_index_x in 0..(rand_indexes_size_x) {
            let mut rand_indexes_y: Vec<usize> = Vec::new();

            let rand_indexes_size_y = (size as f32 / power as f32) as usize;

            for j in 0..rand_indexes_size_y {
                rand_indexes_y.push(j * power);
            }

            rand_indexes_y.push(size as usize - 1);

            if rand_indexes_size_y == 0 {
                continue;
            }

            for current_rand_index_y in 0..rand_indexes_size_y {
                let prev_rand_index_x = rand_indexes_x[current_rand_index_x as usize] as usize;
                let next_rand_index_x = rand_indexes_x[(current_rand_index_x + 1) as usize] as usize;

                let prev_rand_index_y = rand_indexes_y[current_rand_index_y as usize] as usize;
                let next_rand_index_y = rand_indexes_y[(current_rand_index_y + 1) as usize] as usize;
                for x in prev_rand_index_x..next_rand_index_x {
                    if !x_larger && x >= sizex as usize {
                        continue;
                    }

                    for y in prev_rand_index_y..next_rand_index_y {
                        if x_larger && y >= sizey as usize {
                            continue;
                        }

                        if detail {
                            perlin[x as usize][y as usize] += (1.0 / avg_factor) * circle_lerp(
                                randoms
                                    .get_at(prev_rand_index_x as usize, prev_rand_index_y as usize),
                                randoms
                                    .get_at(next_rand_index_x as usize, prev_rand_index_y as usize),
                                randoms
                                    .get_at(prev_rand_index_x as usize, next_rand_index_y as usize),
                                randoms
                                    .get_at(next_rand_index_x as usize, next_rand_index_y as usize),
                                ((x - prev_rand_index_x) as f32)
                                    / ((next_rand_index_x - prev_rand_index_x) as f32),
                                ((y - prev_rand_index_y) as f32)
                                    / ((next_rand_index_y - prev_rand_index_y) as f32),
                            );
                        } else {
                            perlin[x as usize][y as usize] += (power_recipr) * circle_lerp(
                                randoms
                                    .get_at(prev_rand_index_x as usize, prev_rand_index_y as usize),
                                randoms
                                    .get_at(next_rand_index_x as usize, prev_rand_index_y as usize),
                                randoms
                                    .get_at(prev_rand_index_x as usize, next_rand_index_y as usize),
                                randoms
                                    .get_at(next_rand_index_x as usize, next_rand_index_y as usize),
                                ((x - prev_rand_index_x) as f32)
                                    / ((next_rand_index_x - prev_rand_index_x) as f32),
                                ((y - prev_rand_index_y) as f32)
                                    / ((next_rand_index_y - prev_rand_index_y) as f32),
                            );
                        }
                    }
                }
            }
        }
    }

    for i in 0..(sizex) {
        for j in 0..(sizey) {
            if detail {
                perlin[i][j] /= depth as f32; // * avg_factor;
            } else {
                perlin[i][j] /= avg_factor;
            }
        }
    }

    perlin
}
