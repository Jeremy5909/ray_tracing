use std::io::{stdout, Write};

use vec3::{dot, unit_vector};

use crate::{color::{write_color, Color}, ray::Ray, vec3::{Point3, Vec3}};

mod vec3;
mod color;
mod ray;

fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc = center - *r.origin();
    let a = dot(*r.dir(), *r.dir());
    let b = -2.0 * dot(*r.dir(), oc);
    let c = dot(oc, oc) - radius*radius;
    let discriminant = b*b -4.0*a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0*a)
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::from(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0{
        let N: Vec3 = unit_vector(&(r.at(t) - Vec3::from(0.0, 0.0, -1.0)));
        return 0.5*Color::from(N.x()+1.0, N.y()+1.0, N.z()+1.0);
    }

    let unit_direction = unit_vector(r.dir());
    let a = 0.5*(unit_direction.y() + 1.0);
    (1.0-a)*Color::from(1.0, 1.0, 1.0) + a*Color::from(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    //BUG when aspect ratio isn't 1:1, last row in pixels isnt full
    let aspect_ratio: f32 = 1.0;
    let image_width: i32 = 400;

    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let image_height = if image_height < 1 {1} else {image_height};
    
    // Camera
    let focal_length = 1.0f32;
    let viewport_height = 2.0f32;
    let viewport_width = viewport_height * (image_width as f32/image_height as f32);
    let camera_center = Point3::from(0.0, 0.0, 0.0);

    // Vectors across horizontal and vertical viewport edges
    let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

    // Horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u/image_width as f32;
    let pixel_delta_v = viewport_v/image_height as f32;

    let viewport_upper_left = camera_center -
        Vec3::from(0.0, 0.0, focal_length)
        - viewport_u/2.0
        - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3\n{image_width} {image_height}\n255");
    for j in 0..=image_height {
        eprintln!("Scanlines remaining: {}", image_height-j);
        stdout().flush().unwrap();
        for i in 0..image_height {
            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut stdout(), pixel_color);
        }
    }
    eprintln!("Done");
}
