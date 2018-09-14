pub mod interpolators {
    use perlin::bicubic_data_randomizer::bicubic_data_randomizer::BicubicDataRandomizer;

    pub fn cubic(p0: f32, p1: f32, p2: f32, p3: f32, x: f32) -> f32 {
        // (-0.5 * p0 + 1.5 * p1 - 1.5 * p2 + 0.5 * p3) * (x * x * x)
        //     + (p0 - 2.5 * p1 + 2.0 * p2 - 0.5 * p3) * (x * x)
        //     + (-0.5 * p0 + 0.5 * p2) * x
        //     + p1

        p1 + 0.5
            * x
            * (p2 - p0
                + x * (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3 + x * (3.0 * (p1 - p2) + p3 - p0)))
    }

    pub fn cubic_hermite(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
        let aa = -a * 0.5 + (1.5 * b) - (1.5 * c) + d * 0.5;
        let bb = a - (2.5 * b) + 2.0 * c - d * 0.5;
        let cc = -a * 0.5 + c * 0.5;
        let dd = b;

        aa * t * t * t + bb * t * t + cc * t + dd
    }

    fn clamp(x: f32, y: f32, z: f32) -> f32 {
        if x > y {
            x
        } else if z < y {
            z
        } else {
            y
        }
    }

    pub fn bicubic_hermite(source: &[f32; 16], x: f32, y: f32) -> f32 {
        clamp(
            0.0,
            cubic_hermite(
                cubic_hermite(source[0], source[1], source[2], source[3], y), //Tosource[
                cubic_hermite(source[4], source[5], source[6], source[7], y), //Second tosource[
                cubic_hermite(source[8], source[9], source[10], source[11], y), //Second bottom
                cubic_hermite(source[12], source[13], source[14], source[15], y), //Bottom
                x,
            ),
            1.0,
        )
    }

    pub fn bicubic(source: &[f32; 16], x: f32, y: f32) -> f32 {
        clamp(
            0.0,
            cubic(
                cubic(source[0], source[1], source[2], source[3], y), //Tosource[
                cubic(source[4], source[5], source[6], source[7], y), //Second tosource[
                cubic(source[8], source[9], source[10], source[11], y), //Second bottom
                cubic(source[12], source[13], source[14], source[15], y), //Bottom
                x,
            ),
            1.0,
        )
    }

    pub fn easy_bicubic(
        source: &mut BicubicDataRandomizer,
        distx: usize,
        currentx: usize,
        disty: usize,
        currenty: usize,
        x: usize,
        y: usize,
    ) -> f32 {
        // let left = x as isize;
        // let right = (x + distx) as isize;
        // let bottom = (y + disty) as isize;
        // let top = y as isize;
        let arr = &source.get_bicubic_dataset(x, y, distx, disty); //.into_iter().flatten().collect::<Vec<f32>>();
        let distx = distx as f32;
        let disty = disty as f32;
        bicubic(arr, currentx as f32 / distx, currenty as f32 / disty)
    }

    pub fn easy_bicubic_hermite(
        source: &mut BicubicDataRandomizer,
        distx: usize,
        currentx: usize,
        disty: usize,
        currenty: usize,
        x: usize,
        y: usize,
    ) -> f32 {
        // let left = x as isize;
        // let right = (x + distx) as isize;
        // let bottom = (y + disty) as isize;
        // let top = y as isize;
        let arr = &source.get_bicubic_dataset(x, y, distx, disty); //.into_iter().flatten().collect::<Vec<f32>>();
        let distx = distx as f32;
        let disty = disty as f32;
        bicubic_hermite(arr, currentx as f32 / distx, currenty as f32 / disty)
    }

    pub fn lerp(start: f32, end: f32, dist: u32, current: u32) -> f32 {
        let t: f32 = current as f32 / dist as f32;
        return start + ((end - start) * t);
    }

    pub fn inv_sqrt(number: f32) -> f32 {
        let mut i: i64;
        let mut x2: f32;
        let mut y: f32;
        let threehalfs = 1.5;

        unsafe {
            x2 = number * 0.5;
            y = number;
            i = *((&y as *const f32) as *const i64);
            i = 0x5f3759df - (i >> 1);
            y = *((&i as *const i64) as *const f32);
            y = y * (threehalfs - (x2 * y * y));
        }

        y
    }

    pub fn circle_lerp(a: f32, b: f32, c: f32, d: f32, x: f32, y: f32) -> f32 {
        // let sqrt2 = 1.0 / inv_sqrt(2.0);
        // let dista = sqrt2 - 1.0 / inv_sqrt(x*x + y*y);
        // let distb = sqrt2 - 1.0 / inv_sqrt((1.0 - x) * (1.0 - x) + y*y);
        // let distc = sqrt2 - 1.0 / inv_sqrt(x*x + (1.0 - y) * (1.0 - y));
        // let distd = sqrt2 - 1.0 / inv_sqrt((1.0 - x) * (1.0 - x) + (1.0 - y) * (1.0 - y));

        let dista = 1.0 - 1.0 / inv_sqrt(x * x + y * y);
        let distb = 1.0 - 1.0 / inv_sqrt((1.0 - x) * (1.0 - x) + y * y);
        let distc = 1.0 - 1.0 / inv_sqrt(x * x + (1.0 - y) * (1.0 - y));
        let distd = 1.0 - 1.0 / inv_sqrt((1.0 - x) * (1.0 - x) + (1.0 - y) * (1.0 - y));

        let mut val = 0.0;
        let mut count = 0.0;
        if dista >= 0.0 {
            val += a * dista;
            count += 1.0;
        }
        if distb >= 0.0 {
            val += b * distb;
            count += 1.0;
        }
        if distc >= 0.0 {
            val += c * distc;
            count += 1.0;
        }
        if distd >= 0.0 {
            val += d * distd;
            count += 1.0;
        }
        val / 4.0
    }

}
