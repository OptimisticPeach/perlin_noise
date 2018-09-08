pub mod random_2d {
    extern crate rand;
    use rand::Rng;
    pub struct Randomizer2D {
        data: Vec<Vec<f32>>
    }
    impl Randomizer2D {
        pub fn new(width: usize, height: usize) -> Randomizer2D {
            Randomizer2D {
                data: vec![vec![-1.0; height]; width]
            }
        }
        pub fn get_at(&mut self, x: usize, y: usize) -> f32 {
            if self.data[x][y] < 0.0 {
                self.data[x][y] = rand::thread_rng().gen_range(0.0, 1.0);
            }
            self.data[x][y]
        }
        pub fn is_accessed(&self, x: usize, y: usize) -> bool {
            self.data[x][y] >= 0.0
        }
    }
}
