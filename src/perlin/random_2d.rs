pub mod random_2d {
    extern crate rand;
    use rand::Rng;
    pub struct Randomizer2D {
        Indexes: Vec<Vec<isize>>,
        Data: Vec<f32>,
    }
    impl Randomizer2D {
        pub fn new(width: usize, height: usize) -> Randomizer2D {
            let mut object = Randomizer2D {
                Indexes: vec![vec![-1; height]; width],
                Data: Vec::new(),
            };
            object
        }
        pub fn get_at(&mut self, x: usize, y: usize) -> f32 {
            if self.Indexes[x][y] < 0 {
                self.Indexes[x][y] = self.Data.len() as isize;
                self.Data.push(rand::thread_rng().gen_range(0.0, 1.0));
            }
            self.Data[self.Indexes[x][y] as usize]
        }
        pub fn is_accessed(&self, x: usize, y: usize) -> bool {
            self.Indexes[x][y] >= 0
        }
    }
}
