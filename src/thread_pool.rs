use std::{
    f64::NEG_INFINITY,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, ScopedJoinHandle},
};

use crate::{
    color::Color,
    interval::Interval,
    math::{Point3, Vec3},
    object::Object,
    ray::Ray,
    rgb,
    scene::Scene,
    vec3,
};

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub p: (usize, usize),
    pub color: (u8, u8, u8),
}

unsafe impl Send for Pixel {}
unsafe impl Sync for Pixel {}

fn ray_color(r: &Ray, scene: &Scene) -> Color {
    if let Some(hit) = scene.world.hit(r, &Interval::from(0.0)) {
        return 0.5 * (hit.normal + rgb!(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * rgb!(1.0, 1.0, 1.0) + a * rgb!(0.5, 0.7, 1.0);
}

fn render(i: usize, j: usize, scene: &Scene) -> (u8, u8, u8) {
    let pixel_center =
        scene.pixel00_loc + ((i as f64) * scene.pixel_delta_u) + ((j as f64) * scene.pixel_delta_v);
    let ray_direction = pixel_center - scene.camera_center;
    let r = Ray::new(scene.camera_center, ray_direction, 0.0);
    let pixel_color = ray_color(&r, scene);

    pixel_color.to_pixel()
}

pub fn render_threaded(size: usize, scene: &Scene) {
    let (tx, rx) = mpsc::channel::<(usize, usize)>();
    let (result_tx, result_rx) = mpsc::channel::<Pixel>();
    let rx = Arc::new(Mutex::new(rx));
    let result_tx = Arc::new(Mutex::new(result_tx));

    let mut image: Vec<(u8, u8, u8)> = vec![(0, 0, 0); scene.image_width * scene.image_height];

    println!("P3\n{} {}\n255", scene.image_width, scene.image_height);

    thread::scope(|s| {
        for _ in 0..size {
            let rx = Arc::clone(&rx);
            let result_tx = Arc::clone(&result_tx);
            s.spawn(move || {
                while let Ok((i, j)) = rx.lock().unwrap().recv() {
                    let color = render(i, j, scene);

                    result_tx
                        .lock()
                        .unwrap()
                        .send(Pixel { p: (i, j), color })
                        .expect("Failed to send pixel");
                }
            });
        }

        for j in 0..scene.image_height {
            for i in 0..scene.image_width {
                tx.send((i, j)).expect("Failed to send pixel");
            }
        }

        drop(tx);

        for i in 0..(image.len()) {
            eprint!(
                "\rProgress: {}% ",
                (((i as f64) / (image.len() as f64)) * 100.0) as u8
            );
            let pixel = result_rx.recv().expect("Failed to receive pixel");
            image[pixel.p.0 + pixel.p.1 * scene.image_width] = pixel.color;
        }

        drop(result_rx);
    });

    eprintln!("\rDone.                   ");

    for color in image {
        println!("{} {} {}", color.0, color.1, color.2);
    }
}

pub fn render_unthreaded(scene: &Scene) {
    println!("P3\n{} {}\n255", scene.image_width, scene.image_height);

    for j in 0..scene.image_height {
        for i in 0..scene.image_width {
            eprint!(
                "\rProgress: {}% ",
                (((j as f64) / (scene.image_height as f64)) * 100.0) as u8
            );
            let color = render(i, j, scene);
            println!("{} {} {}", color.0, color.1, color.2);
        }
    }

    eprintln!("\rDone.                   ");
}
