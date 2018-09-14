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

    pub fn circle_lerp(a: f32, b: f32, c: f32, d: f32, x: f32, y: f32) -> f32 {
        let dista = 1.0 - (x * x + y * y).sqrt();
        let distb = 1.0 - ((1.0 - x) * (1.0 - x) + y * y).sqrt();
        let distc = 1.0 - (x * x + (1.0 - y) * (1.0 - y)).sqrt();
        let distd = 1.0 - ((1.0 - x) * (1.0 - x) + (1.0 - y) * (1.0 - y)).sqrt();

        let mut val = 0.0;
        if dista >= 0.0 {
            val += a * dista;
        }
        if distb >= 0.0 {
            val += b * distb;
        }
        if distc >= 0.0 {
            val += c * distc;
        }
        if distd >= 0.0 {
            val += d * distd;
        }
        val
    }

}
