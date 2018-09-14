#![allow(dead_code)]
#![allow(unused_imports)]

extern crate image;
extern crate rand;
mod perlin;

use image::{ImageBuffer};
use perlin::perlin::{get_perlin_2d, fill_rand_2d};
use perlin::interpolators::*;

const IMG_SIZE_Y: u32 = 2048;
const IMG_SIZE_X: u32 = 1024;

fn main() {

	let perlin_data = get_perlin_2d(IMG_SIZE_X as usize, IMG_SIZE_Y as usize, Some(10));

	let mut buffer: Vec<u8> = Vec::new();

	for i in 0..IMG_SIZE_X{
		for j in 0..IMG_SIZE_Y{
			let color = (perlin_data[i as usize][j as usize] * 255.0) as u8;

			buffer.push(color);
		}
	} 
	

	let img = ImageBuffer::<image::Luma<u8>, std::vec::Vec<u8>>::from_vec(IMG_SIZE_Y, IMG_SIZE_X, buffer).unwrap();

	img.save("perlin.png").unwrap();
	// let c = 20000000;
	// let start = std::time::Instant::now();
	// let mut v = vec![0.0; c];
	// for i in 0..c
	// {v[i] = interpolators::inv_sqrt(16.0);}
	// println!("{}", start.elapsed().as_secs() as f32 + start.elapsed().subsec_nanos() as f32 / 1_000_000_000.0);
}
   