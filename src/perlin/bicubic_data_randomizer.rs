pub mod bicubic_data_randomizer {
    extern crate rand;
    use rand::Rng;

    pub enum Corner {
        TopLeft,
        TopRight,
        BottomLeft,
        BottomRight,
    }

    #[derive(Copy)]
    struct SmartSquare {
        tl: f32, //Top left, original
        tr: f32, //Top right
        bl: f32, //Bottom left
        br: f32, //Bottom right
    }

    impl SmartSquare {
        ///Returns new corners: top left, top right, bottom left, bottom right
        pub fn get_as_corner(&self, choice: Corner) -> (f32, f32, f32, f32) {
            match choice {
                Corner::TopLeft => (self.br, self.bl, self.tr, self.tl),
                Corner::TopRight => (self.tr, self.br, self.tl, self.bl),
                Corner::BottomLeft => (self.bl, self.tl, self.br, self.tr),
                Corner::BottomRight => (self.tl, self.tr, self.bl, self.br),
            }
        }

        pub fn new(tl_: f32, tr_: f32, bl_: f32, br_: f32) -> SmartSquare {
            SmartSquare {
                tl: tl_,
                tr: tr_,
                bl: bl_,
                br: br_,
            }
        }
    }

    pub struct BicubicDataRandomizer {
        data: Vec<Vec<SmartSquare>>,
        width_main: usize,
        height_main: usize,
    }

    impl BicubicDataRandomizer {
        pub fn new(width: usize, height: usize) -> BicubicDataRandomizer {
            BicubicDataRandomizer {
                data: vec![vec![SmartSquare::new(-1.0, 0.0, 0.0, 0.0); height]; width],
                width_main: width as usize,
                height_main: height as usize,
            }
        }

        fn set_if_not(&mut self, x: usize, y: usize) {
            if self.data[x][y].get_as_corner(Corner::BottomRight).0 == -1.0 {
                self.data[x][y] = SmartSquare::new(
                    rand::thread_rng().gen_range(0.0, 1.0),
                    rand::thread_rng().gen_range(0.0, 1.0),
                    rand::thread_rng().gen_range(0.0, 1.0),
                    rand::thread_rng().gen_range(0.0, 1.0),
                );
            }
        }

        pub fn get_bicubic_dataset(
            &mut self,
            x: usize,
            y: usize,
            distx: usize,
            disty: usize,
        ) -> [f32; 16] {
            self.set_if_not(x, y);
            self.set_if_not(x + distx, y);
            self.set_if_not(x, y + disty);
            self.set_if_not(x + distx, y + disty);

            let tl = self.data[x][y].get_as_corner(Corner::TopLeft);
            let tr = self.data[x + distx][y].get_as_corner(Corner::TopRight);
            let bl = self.data[x][y + disty].get_as_corner(Corner::BottomLeft);
            let br = self.data[x + distx][y + disty].get_as_corner(Corner::BottomRight);

            [
                tl.0, tl.1, tr.0, tr.1, tl.2, tl.3, tr.2, tr.3, bl.0, bl.1, br.0, br.1, bl.2, bl.3,
                bl.2, bl.3,
            ]
        }
    }

    impl Clone for SmartSquare {
        fn clone(&self) -> SmartSquare {
            *self
        }
    }

}
