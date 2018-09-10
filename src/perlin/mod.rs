mod random_2d;
pub mod perlin {
    extern crate rand;
    use perlin::random_2d::random_2d::{access_2d_percent, Randomizer2D};
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

    fn cubic(p0: f32, p1: f32, p2: f32, p3: f32, x: f32) -> f32 {
        (-0.5 * p0 + 1.5 * p1 - 1.5 * p2 + 0.5 * p3) * (x * x * x)
            + (p0 - 2.5 * p1 + 2.0 * p2 - 0.5 * p3) * (x * x)
            + (-0.5 * p0 + 0.5 * p2) * x
            + p1
    }

    fn bicubic(source: &Vec<Vec<f32>>, x: f32, y: f32) -> f32 {
        cubic(
            cubic(source[0][0], source[1][0], source[2][0], source[3][0], y), //Tosource[
            cubic(source[0][1], source[1][1], source[2][1], source[3][1], y), //Second tosource[
            cubic(source[0][2], source[1][2], source[2][2], source[3][2], y), //Second bottom
            cubic(source[0][3], source[1][3], source[2][3], source[3][3], y), //Bottom
            x,
        )
    }

    fn easy_bicubic<T>(
        source: &mut T,
        distx: usize,
        currentx: usize,
        disty: usize,
        currenty: usize,
        x: usize,
        y: usize,
    ) -> f32
    where
        T: access_2d_percent,
    {
        let left = x as isize;
        let right = (x + distx) as isize;
        let bottom = (y + disty) as isize;
        let top = y as isize;
        let (width, height) = source.get_size();
        let arr = &source.get_rect(x as isize - 1, y as isize - 1, 4, 4, distx, disty);//.into_iter().flatten().collect::<Vec<f32>>();
        let distx = distx as f32;
        let disty = disty as f32;
        bicubic(arr, currentx as f32 / distx, currenty as f32 / disty)
    }

    pub fn get_perlin_2d(sizex: usize, sizey: usize, depth: Option<u32>) -> Vec<Vec<f32>> {
        let size = cmp::max(sizex + 1, sizey + 1); //There's a bug that doesn't allow non-square perlin noise generation...
                                                   //But we can mitigate that by ignoring the values beyond the limits set,
                                                   //And using the random_2d struct to only generate necessary values at
                                                   //The cost of using some memory

        let x_larger = cmp::max(sizex, sizey) == sizex;

        let depth = depth.unwrap_or(std::u32::MAX);
        let mut randoms = Randomizer2D::new(size, size);
        let mut perlin: Vec<Vec<f32>> = vec![vec![0.0; size]; size];

        let size_log2 = (size as f32).log2();

        let depth = if depth < size_log2.floor() as u32 {
            depth
        } else {
            size_log2.floor() as u32
        };

        let mut avg_factor = 0.0;

        for octave_index in (0..depth).rev() {
            let power = (octave_index as f32).exp2() as u32;
            let power_recipr = 1.0 / ((depth - octave_index) as f32).exp2();
            avg_factor += power_recipr;

            let mut rand_indexes_x: Vec<u32> = Vec::new();

            let rand_indexes_size_x = ((size as u32 / power) - 1) as u32;

            for j in 0..rand_indexes_size_x {
                rand_indexes_x.push(j * power);
            }

            rand_indexes_x.push(size as u32 - 1);

            if rand_indexes_size_x == 0 {
                continue;
            }

            for current_rand_index_x in 0..(rand_indexes_size_x) {
                let mut rand_indexes_y: Vec<u32> = Vec::new();

                let rand_indexes_size_y = ((size as u32 / power) - 1) as u32;

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

                            perlin[x as usize][y as usize] += power_recipr
                                * easy_bicubic(
                                    &mut randoms,
                                    (next_rand_index_x - prev_rand_index_x) as usize,
                                    (x - prev_rand_index_x) as usize,
                                    (next_rand_index_y - prev_rand_index_y) as usize,
                                    (y - prev_rand_index_y) as usize,
                                    prev_rand_index_x as usize,
                                    prev_rand_index_y as usize,
                                )

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
                perlin[i][j] /= avg_factor;
            }
        }

        perlin
    }
}
