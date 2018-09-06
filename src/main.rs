extern crate image;
extern crate rand;
mod perlin;

use image::{ImageBuffer};
use perlin::perlin::{get_perlin_2d};

const IMG_SIZE: u32 = 512;

fn main() {
	// let perlin_data = perlin::perlin::get_perlin(512, Some(8));

	// for (index, b) in perlin_data.iter().enumerate(){
	// 	let size = (b * 100.0) as u32;
	// 	print!("{} | {}  ", format!("{:04}", index), format!("{:.*}", 8, b));
	// 	for _ in 0..size{
	// 		print!("\u{2593}")
	// 	}
	// 	println!("");
	// }

	let perlin_data = get_perlin_2d(IMG_SIZE as usize, IMG_SIZE as usize, None);

	let mut buffer: Vec<u8> = Vec::new();

	for i in 0..IMG_SIZE{
		for j in 0..IMG_SIZE{
			let color = (perlin_data[i as usize][j as usize] * 255.0) as u8;
			let mut color = [color, color, color, 255_u8];//(perlin_data[x as usize][y as usize] * 255.0) as u8;

			buffer.extend(color.iter());
		}
	} 
	

	let img = ImageBuffer::<image::Rgba<u8>, std::vec::Vec<u8>>::from_vec(IMG_SIZE, IMG_SIZE, buffer).unwrap();

	//let img = ImageBuffer::from_fn(IMG_SIZE, IMG_SIZE, |x, y| {image::Luma([0u8])});

	//let b: u8 = img;

	img.save("perlin.png").unwrap();

	// for y_line in perlin_data{
	// 	for character in y_line.iter(){
	// 		if character < &0.33{
	// 			print!("\u{2591}");
	// 		}
	// 		else if character < &0.66{
	// 			print!("\u{2592}");
	// 		}
	// 		else{
	// 			print!("\u{2593}");
	// 		}
	// 	}
	// 	println!("");
	// }
}
