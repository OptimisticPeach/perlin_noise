#![allow(dead_code)]
#![allow(unused_imports)]

extern crate image;
extern crate rand;
mod perlin;

use image::ImageBuffer;
use rand::Rng;
use perlin::interpolators::*;
use perlin::{fill_rand_2d, get_perlin_2d};
use std::time::Instant;

const IMG_SIZE_Y: u32 = 1024;
const IMG_SIZE_X: u32 = 1024;

fn main() {
	let perlin_data = get_perlin_2d(
		IMG_SIZE_X as usize,
		IMG_SIZE_Y as usize,
		Some(10),
		Some(false),
		|| {rand::thread_rng().gen_range(0.0, 1.0)}	);

	let mut buffer: Vec<u8> = Vec::new();

	for i in 0..IMG_SIZE_X {
		for j in 0..IMG_SIZE_Y {
			let color = (perlin_data[i as usize][j as usize] * 255.0) as u8;

			buffer.push(color);
		}
	}

	let img =
		ImageBuffer::<image::Luma<u8>, std::vec::Vec<u8>>::from_vec(IMG_SIZE_Y, IMG_SIZE_X, buffer)
			.unwrap();

	img.save("perlin.png").unwrap();
}
