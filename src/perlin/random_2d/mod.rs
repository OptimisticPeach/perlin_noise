pub mod random_2d {

    extern crate rand;

    use rand::Rng;

    pub struct randomizer_2d {
        Indexes: Vec<Vec<isize>>,
        Data: Vec<f32>,
    }
    impl randomizer_2d {
        pub fn new(width: usize, height: usize) -> randomizer_2d {
            let mut object = randomizer_2d {
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

        pub fn get_all_accessed(&mut self) -> Vec<(usize, usize)>{
            let mut vec: Vec<(usize, usize)> = Vec::new();
            for x in 0..self.Indexes.len(){
                for y in 0..self.Indexes[0].len(){
                    if self.Indexes[x][y] != -1{
                        vec.push((x, y));
                    }
                }
            }

            vec
        }

        pub fn is_accessed(&self, x: usize, y: usize) -> bool{
            self.Indexes[x][y] >= 0
        }
    }
}
