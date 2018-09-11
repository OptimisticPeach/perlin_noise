pub mod random_2d {
    extern crate rand;
    use rand::Rng;

    pub struct Randomizer2D {
        data: Vec<Vec<f32>>,
        width_main: isize,
        height_main: isize,
    }

    pub trait Access2dPercent {
        fn get_at(&mut self, x: isize, y: isize) -> f32;
        fn get_size(&self) -> (usize, usize);
        fn get_line(&mut self, x: isize, y: isize, len: usize, direction_is_x: bool, dist: usize) -> Vec<f32>;
        fn get_rect(&mut self, x: isize, y: isize, width: usize, height: usize, distx: usize, disty: usize) -> Vec<Vec<f32>>;
    }
    impl Access2dPercent for Randomizer2D {
        fn get_at(&mut self, x: isize, y: isize) -> f32 {

            let newx = x + self.width_main;
            let newy = y + self.height_main;

            if self.data[newx as usize][newy as usize] < 0.0 {
                self.data[newx as usize][newy as usize] = rand::thread_rng().gen_range(0.0, 1.0);
            }
            self.data[newx as usize][newy as usize]
        }
        fn get_size(&self) -> (usize, usize) {
            (self.data.len(), self.data[0].len())
        }
        fn get_line(&mut self, x: isize, y: isize, len: usize, direction_is_x: bool, dist: usize) -> Vec<f32> {
            let mut line: Vec<f32> = Vec::new();
            for i in 0..len as isize {
                if direction_is_x {
                    line.push(self.get_at(x + i * dist as isize, y));
                }
                else{
                    line.push(self.get_at(x, y + i * dist as isize));
                }
            }
            line
        }
        fn get_rect(&mut self, x: isize, y: isize, width: usize, height: usize, distx: usize, disty: usize) -> Vec<Vec<f32>> {
            let mut rect: Vec<Vec<f32>> = Vec::new();
            for i in 0..width as isize {
                rect.push(self.get_line(x + i * distx as isize, y, height, false, disty));
            }
            rect
        }
    }
    impl Randomizer2D {
        pub fn new(width: usize, height: usize) -> Randomizer2D {
            Randomizer2D {
                data: vec![vec![-1.0; height * 3]; width * 3],
                width_main: width as isize,
                height_main: height as isize,
            }
        }

        pub fn is_accessed(&self, x: isize, y: isize) -> bool {
            let x = (x + self.width_main) as usize;
            let y = (y + self.height_main) as usize;
            self.data[x][y] >= 0.0
        }
    }

}
