mod random_2d;
mod bicubic_data_randomizer;
pub mod interpolators;
//mod interpolators;
pub mod perlin {
    extern crate rand;
    use perlin::random_2d::random_2d::{Access2dPercent, Randomizer2D};
    use perlin::bicubic_data_randomizer::bicubic_data_randomizer::BicubicDataRandomizer;
    use rand::Rng;
    use std;
    use std::cmp;
    use std::f32;
    use std::ops::Range;
    use perlin::interpolators::interpolators::*;

    pub fn fill_rand(size: usize) -> Vec<f32> {
        let mut randoms: Vec<f32> = Vec::new();

        for _ in 0..size {
            randoms.push(rand::thread_rng().gen_range(0.0, 1.0));
        }

        randoms
    }

    pub fn get_perlin(size: usize, depth: Option<u32>) -> Vec<f32> {
        let depth = depth.unwrap_or(std::u32::MAX);
        let randoms = fill_rand(size);
        let mut perlin: Vec<f32> = vec![0.0; size];

        let size_log2 = (size as f32).log2();

        let depth = if depth < size_log2.floor() as u32 {
            depth
        } else {
            size_log2.floor() as u32
        };

        let depth = depth as u32;

        let mut recipr = 0;

        let mut avg_factor = 0.0;

        for depth_index in (0..depth).rev() {
            let power = (depth_index as f32).exp2() as u32;
            let power_recipr = 1.0 / (recipr as f32).exp2();
            avg_factor += power_recipr;

            recipr += 1;

            let mut rand_indexes: Vec<u32> = Vec::new();

            let rand_indexes_size = (size as u32 / power) as u32;

            for j in 0..rand_indexes_size {
                rand_indexes.push(j * power);
            }

            rand_indexes.push(size as u32 - 1);

            if rand_indexes_size == 0 {
                continue;
            }

            for current_rand_index in 0..(rand_indexes_size) {
                let prev_rand_index = rand_indexes[current_rand_index as usize] as u32;
                let next_rand_index = rand_indexes[(current_rand_index + 1) as usize] as u32;
                for i in prev_rand_index..next_rand_index {
                    perlin[i as usize] += power_recipr * lerp(
                        randoms[prev_rand_index as usize],
                        randoms[next_rand_index as usize],
                        next_rand_index - prev_rand_index,
                        i - prev_rand_index,
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

    

    pub fn get_perlin_2d(sizex: usize, sizey: usize, depth: Option<u32>) -> Vec<Vec<f32>> {
        let size = cmp::max(sizex + 1, sizey + 1);

        let x_larger = cmp::max(sizex, sizey) == sizex;

        let depth = depth.unwrap_or(std::u32::MAX);
        // let mut randoms = BicubicDataRandomizer::new(size, size);
        let mut randoms = Randomizer2D::new(size, size);
        let mut perlin: Vec<Vec<f32>> = vec![vec![0.0; size]; size];

        let size_log2 = (size as f32).log2();

        let max_depth = size_log2.ceil() as u32;
        let depth = if depth < max_depth {
            depth
        } else {
            max_depth
        };

        let depth_power_list = (0..=depth)        
        .map(|x| (((max_depth - x) as f32)
        .exp2() as u32, 1.0 / (x as f32).exp2()));

        let mut avg_factor = 0.0;

        for (power, power_recipr) in depth_power_list {
            
            avg_factor += power_recipr;

            let mut rand_indexes_x: Vec<u32> = Vec::new();

            let rand_indexes_size_x = (size as f32 / power as f32) as u32;

            for j in 0..rand_indexes_size_x {
                rand_indexes_x.push(j * power);
            }

            rand_indexes_x.push(size as u32 - 1);

            if rand_indexes_size_x == 0 {
                continue;
            }

            for current_rand_index_x in 0..(rand_indexes_size_x) {
                let mut rand_indexes_y: Vec<u32> = Vec::new();

                let rand_indexes_size_y = (size as f32 / power as f32) as u32;

                for j in 0..rand_indexes_size_y {
                    rand_indexes_y.push(j * power); //+1 to compensate for the bicubic
                }

                rand_indexes_y.push(size as u32 - 1);

                if rand_indexes_size_y == 0 {
                    continue;
                }

                for current_rand_index_y in 0..rand_indexes_size_y {
                    let prev_rand_index_x = rand_indexes_x[current_rand_index_x as usize] as u32;
                    let next_rand_index_x =
                        rand_indexes_x[(current_rand_index_x + 1) as usize] as u32;

                    let prev_rand_index_y = rand_indexes_y[current_rand_index_y as usize] as u32;
                    let next_rand_index_y =
                        rand_indexes_y[(current_rand_index_y + 1) as usize] as u32;
                    // let top_left =
                    //     randoms.get_at(prev_rand_index_x as isize, prev_rand_index_y as isize);
                    // let top_right =
                    //     randoms.get_at(next_rand_index_x as isize, prev_rand_index_y as isize);
                    // let bottom_left =
                    //     randoms.get_at(prev_rand_index_x as isize, next_rand_index_y as isize);
                    // let bottom_right =
                    //     randoms.get_at(next_rand_index_x as isize, next_rand_index_y as isize);
                    for x in prev_rand_index_x..next_rand_index_x {
                        if !x_larger && x >= sizex as u32 {
                            continue;
                        }

                        for y in prev_rand_index_y..next_rand_index_y {
                            if x_larger && y >= sizey as u32 {
                                continue;
                            }

                            perlin[x as usize][y as usize] += power_recipr * circle_lerp(
                                randoms.get_at(prev_rand_index_x as usize, prev_rand_index_y as usize),
                                randoms.get_at(next_rand_index_x as usize, prev_rand_index_y as usize),
                                randoms.get_at(prev_rand_index_x as usize, next_rand_index_y as usize),
                                randoms.get_at(next_rand_index_x as usize, next_rand_index_y as usize),
                                ((x - prev_rand_index_x) as f32) / ((next_rand_index_x - prev_rand_index_x) as f32),
                                ((y - prev_rand_index_y) as f32) / ((next_rand_index_y - prev_rand_index_y) as f32)
                            );

                            // perlin[x as usize][y as usize] += power_recipr * easy_bicubic(
                            //     &mut randoms,
                            //     (next_rand_index_x - prev_rand_index_x) as usize,
                            //     (x - prev_rand_index_x) as usize,
                            //     (next_rand_index_y - prev_rand_index_y) as usize,
                            //     (y - prev_rand_index_y) as usize,
                            //     prev_rand_index_x as usize,
                            //     prev_rand_index_y as usize,
                            // )

                            // let x_start = lerp(
                            //     top_left,
                            //     bottom_left,
                            //     next_rand_index_y - prev_rand_index_y,
                            //     y - prev_rand_index_y,
                            // );
                            // let x_end = lerp(
                            //     top_right,
                            //     bottom_right,
                            //     next_rand_index_y - prev_rand_index_y,
                            //     y - prev_rand_index_y,
                            // );
                            // perlin[x as usize][y as usize] += power_recipr
                            //     * lerp(
                            //         x_start,                               //start_x
                            //         x_end,                                 //end_y
                            //         next_rand_index_x - prev_rand_index_x, //dist_x
                            //         x - prev_rand_index_x,                 //current_x
                            //     );
                        }
                    }
                }
            }
        }

        for i in 0..(sizex) {
            for j in 0..(sizey) {
                perlin[i][j] /= avg_factor / 4.0;
            }
        }

        let mut max = 0.0;

        for i in 0..(sizex) {
            for j in 0..(sizey) {
                if max < perlin[i][j]{
                    max = perlin[i][j];
                }
            }
        }
        
        println!("{}", max);

        perlin
    }
}
