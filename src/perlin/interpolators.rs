pub mod interpolators {

    pub fn lerp(start: f32, end: f32, percent: f32) -> f32 {
        return start + ((end - start) * percent);
    }

    pub fn bilinear(a: f32, b: f32, c: f32, d: f32, x: f32, y: f32) -> f32{
        lerp(
            lerp(a, b, x),
            lerp(c, d, x),
            y
        )
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
