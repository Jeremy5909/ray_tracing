use std::io::{stdout, Write};

use crate::color::{write_color, Color};

mod vec3;
mod color;
mod ray;


fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    
    let image_width: i32 = 400;

    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let image_height = if image_height < 1 {1} else {image_height};

    let viewport_height = 2.0f32;
    let viewport_width = viewport_height * (image_width/image_height) as f32;



    println!("P3\n{image_width} {image_height}\n255");
    for j in 0..=image_height {
        eprintln!("Scanlines remaining: {}", image_height-j);
        stdout().flush().unwrap();
        for i in 0..image_height {
            let pixel_color = Color::from(
                (i as f32)/(image_width as f32-1.0),
                (j as f32)/(image_height as f32-1.0),
                0.0);
            write_color(&mut stdout(), pixel_color);
        }
    }
    eprintln!("Done");
}
