extern crate rand;
use rand::Rng;

pub struct Randomizer2D<RngGen: Fn() -> f32> {
    data: Vec<Vec<f32>>,
    width_main: usize,
    height_main: usize,
    func: RngGen
}

pub trait Access2dPercent {
    fn get_at(&mut self, x: usize, y: usize) -> f32;
    fn get_size(&self) -> (usize, usize);
    fn get_line(
        &mut self,
        x: usize,
        y: usize,
        len: usize,
        direction_is_x: bool,
        dist: usize,
    ) -> Vec<f32>;
    fn get_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        distx: usize,
        disty: usize,
    ) -> Vec<Vec<f32>>;
}
impl<T: Fn() -> f32> Access2dPercent for Randomizer2D<T> {
    fn get_at(&mut self, x: usize, y: usize) -> f32 {
        if self.data[x][y] < 0.0 {
            self.data[x][y] = (self.func)();
        }
        self.data[x][y]
    }
    fn get_size(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }
    fn get_line(
        &mut self,
        x: usize,
        y: usize,
        len: usize,
        direction_is_x: bool,
        dist: usize,
    ) -> Vec<f32> {
        let mut line: Vec<f32> = Vec::new();
        for i in 0..len as usize {
            if direction_is_x {
                line.push(self.get_at(x + i * dist as usize, y));
            } else {
                line.push(self.get_at(x, y + i * dist as usize));
            }
        }
        line
    }
    fn get_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        distx: usize,
        disty: usize,
    ) -> Vec<Vec<f32>> {
        let mut rect: Vec<Vec<f32>> = Vec::new();
        for i in 0..width as usize {
            rect.push(self.get_line(x + i * distx as usize, y, height, false, disty));
        }
        rect
    }
}
impl<T: Fn() -> f32> Randomizer2D<T>
where T: Fn() -> f32 {
    pub fn new(width: usize, height: usize, f: T) -> Randomizer2D<T> {
        Randomizer2D {
            data: vec![vec![-1.0; height]; width],
            width_main: width as usize,
            height_main: height as usize,
            func: f
        }
    }

    pub fn is_accessed(&self, x: usize, y: usize) -> bool {
        let x = (x + self.width_main) as usize;
        let y = (y + self.height_main) as usize;
        self.data[x][y] >= 0.0
    }
}
