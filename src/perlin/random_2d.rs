pub mod random_2d {
    extern crate rand;
    use rand::Rng;

    pub struct Randomizer2D {
        data: Vec<Vec<f32>>,
        width_main: isize,
        height_main: isize
    }

    pub trait access_2d_percent {
        fn get_at(&mut self, x: isize, y: isize) -> f32;
        fn get_size(&self) -> (usize, usize);
    }
    impl access_2d_percent for Randomizer2D {
        fn get_at(&mut self, x: isize, y: isize) -> f32 {

            let x = (x + self.width_main) as usize;
            let y = (y + self.height_main) as usize;

            if self.data[x][y] < 0.0 {
                self.data[x][y] = rand::thread_rng().gen_range(0.0, 1.0);
            }
            self.data[x][y]
        }
        fn get_size(&self) -> (usize, usize){
            (self.data.len(), self.data[0].len())
        }
    }
    impl Randomizer2D {
        pub fn new(width: usize, height: usize) -> Randomizer2D {
            Randomizer2D {
                data: vec![vec![-1.0; height * 3]; width * 3],
                width_main: width as isize,
                height_main: height as isize
            }
        }

        pub fn is_accessed(&self, x: isize, y: isize) -> bool {
            let x = (x + self.width_main) as usize;
            let y = (y + self.height_main) as usize;
            self.data[x][y] >= 0.0
        }
    }

}
