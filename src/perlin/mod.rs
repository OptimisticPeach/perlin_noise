mod random_2d;
pub mod perlin {
    extern crate rand;

    use perlin::random_2d::random_2d::randomizer_2d;
    use rand::Rng;
    use std;
    use std::cmp;
    use std::f32;

    fn lerp(start: f32, end: f32, dist: u32, current: u32) -> f32 {
        let t: f32 = current as f32 / dist as f32;
        return start + ((end - start) * t);
    }

    pub fn fill_rand(size: usize) -> Vec<f32> {
        let mut randoms: Vec<f32> = Vec::new();

        for _ in 0..size {
            randoms.push(rand::thread_rng().gen_range(0.0, 1.0));
        }

        randoms
    }

    pub fn get_perlin(size: usize, depth: Option<u32>) -> Vec<f32> {
        let size = size + 2;
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

            let rand_indexes_size = ((size as u32 / power) - 1) as u32;

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
                    perlin[i as usize] += power_recipr
                        * lerp(
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

    fn lerp_2d(
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        dist_x: u32,
        current_x: u32,
        dist_y: u32,
        current_y: u32,
    ) -> f32 {
        return (lerp(start_x, end_x, dist_x, current_x) + lerp(start_y, end_y, dist_y, current_y))
            / 2.0;
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

    pub fn get_perlin_2d(sizex: usize, sizey: usize, depth: Option<u32>) -> (Vec<Vec<f32>>, randomizer_2d) {
        let sizex = sizex + 2;
        let sizey = sizey + 2;
        let depth = depth.unwrap_or(std::u32::MAX);
        let mut randoms = randomizer_2d::new(sizex, sizey);//fill_rand_2d(sizex, sizey);
        let mut perlin: Vec<Vec<f32>> = vec![vec![0.0; sizey]; sizex];

        let sizex_log2 = (sizex as f32).log2();
        let sizey_log2 = (sizex as f32).log2();

        let depth = if depth < cmp::min(sizex_log2.floor() as u32, sizey_log2.floor() as u32) {
            depth
        } else {
            cmp::min(sizex_log2.floor() as u32, sizey_log2.floor() as u32)
        };

        let mut avg_factor = 0.0;

        for depth_index in (0..depth).rev() {
            let power = (depth_index as f32).exp2() as u32;
            let power_recipr = 1.0 / ((depth - depth_index) as f32).exp2();
            avg_factor += power_recipr;

            let mut rand_indexes_x: Vec<u32> = Vec::new();

            let rand_indexes_size_x = ((sizex as u32 / power) - 1) as u32;

            for j in 0..rand_indexes_size_x {
                rand_indexes_x.push(j * power);
            }

            rand_indexes_x.push(sizex as u32 - 1);

            if rand_indexes_size_x == 0 {
                continue;
            }

            for current_rand_index_x in 0..(rand_indexes_size_x) {
                let mut rand_indexes_y: Vec<u32> = Vec::new();

                let rand_indexes_size_y = ((sizey as u32 / power) - 1) as u32;

                for j in 0..rand_indexes_size_y {
                    rand_indexes_y.push(j * power);
                }

                rand_indexes_y.push(sizey as u32 - 1);

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

                    for x in prev_rand_index_x..next_rand_index_x {

                        let top_left =
                            randoms.get_at(prev_rand_index_x as usize, prev_rand_index_y as usize);
                        let top_right =
                            randoms.get_at(next_rand_index_x as usize, prev_rand_index_y as usize);
                        let bottom_left =
                            randoms.get_at(prev_rand_index_x as usize, next_rand_index_y as usize);
                        let bottom_right =
                            randoms.get_at(next_rand_index_x as usize, next_rand_index_y as usize);

                        for y in prev_rand_index_y..next_rand_index_y {
                            let x_start = lerp(
                                top_left,
                                bottom_left,
                                next_rand_index_y - prev_rand_index_y,
                                y - prev_rand_index_y,
                            );
                            let x_end = lerp(
                                top_right,
                                bottom_right,
                                next_rand_index_y - prev_rand_index_y,
                                y - prev_rand_index_y,
                            );
                            perlin[x as usize][y as usize] += power_recipr
                                * lerp(
                                    x_start,                               //start_x
                                    x_end,                                 //end_y
                                    next_rand_index_x - prev_rand_index_x, //dist_x
                                    x - prev_rand_index_x,                 //current_x
                                );

                            // lerp_2d(
                            //     randoms[prev_rand_index_x as usize][y as usize], //start_x
                            //     randoms[x as usize][prev_rand_index_y as usize], //start_y
                            //     randoms[next_rand_index_x as usize][y as usize], //end_x
                            //     randoms[x as usize][next_rand_index_y as usize], //end_y
                            //     next_rand_index_x - prev_rand_index_x,           //dist_x
                            //     x - prev_rand_index_x,                           //current_x
                            //     next_rand_index_y - prev_rand_index_y,           //dist_y
                            //     y - prev_rand_index_y,                           //current_y
                            // );
                        }
                    }
                }
            }
        }

        for i in 0..(sizex - 1) {
            let mut line = vec![0.0; sizey];
            for j in 0..(sizey - 1) {
                line[j] = perlin[i][j] / avg_factor;
            }
            perlin[i] = line;
        }

        (perlin[0..(sizex - 1)].to_vec(), randoms)
    }
}
