use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use crate::{
    color::Color,
    math::{Point3, Vec3},
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

pub struct ThreadPool {
    pub tx: Sender<(usize, usize)>,
    pub rx: Receiver<Pixel>,
    pub handles: Vec<JoinHandle<()>>,
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin;
    let a = r.direction.length_squared();
    let h = r.direction.dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&vec3!(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = r.at(t) - vec3!(0.0, 0.0, -1.0);
        return 0.5 * rgb!(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * rgb!(1.0, 1.0, 1.0) + a * rgb!(0.5, 0.7, 1.0);
}

fn render(i: usize, j: usize, scene: &Arc<Scene>) -> (u8, u8, u8) {
    let pixel_center =
        scene.pixel00_loc + ((i as f64) * scene.pixel_delta_u) + ((j as f64) * scene.pixel_delta_v);
    let ray_direction = pixel_center - scene.camera_center;
    let r = Ray::new(scene.camera_center, ray_direction, 0.0);
    let pixel_color = ray_color(&r);

    pixel_color.to_pixel()
}

impl ThreadPool {
    pub fn new(size: usize, scene: &Arc<Scene>) -> Self {
        let (tx, rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let result_tx = Arc::new(Mutex::new(result_tx));

        let mut handles = vec![];

        for _ in 0..size {
            let scene = Arc::clone(&scene);
            let rx = Arc::clone(&rx);
            let result_tx = Arc::clone(&result_tx);
            let handle = thread::spawn(move || {
                while let Ok((i, j)) = rx.lock().unwrap().recv() {
                    let color = render(i, j, &scene);

                    result_tx
                        .lock()
                        .unwrap()
                        .send(Pixel { p: (i, j), color })
                        .expect("Failed to send pixel");
                }
            });
            handles.push(handle);
        }

        Self {
            tx,
            rx: result_rx,
            handles,
        }
    }
}

pub fn render_threaded(size: usize, scene: &Arc<Scene>) {
    let pool = ThreadPool::new(size, &scene);

    let mut image: Vec<(u8, u8, u8)> = vec![(0, 0, 0); scene.image_width * scene.image_height];

    println!("P3\n{} {}\n255", scene.image_width, scene.image_height);

    for j in 0..scene.image_height {
        for i in 0..scene.image_width {
            pool.tx.send((i, j)).expect("Failed to send pixel");
        }
    }

    drop(pool.tx);

    for i in 0..(image.len()) {
        eprint!(
            "\rProgress: {}% ",
            (((i as f64) / (image.len() as f64)) * 100.0) as u8
        );
        let pixel = pool.rx.recv().expect("Failed to receive pixel");
        image[pixel.p.0 + pixel.p.1 * scene.image_width] = pixel.color;
    }

    for handle in pool.handles {
        handle.join().unwrap();
    }

    eprintln!("\rDone.                   ");

    for pixel in pool.rx {
        image[pixel.p.0 + pixel.p.1 * scene.image_width] = pixel.color;
    }

    for color in image {
        println!("{} {} {}", color.0, color.1, color.2);
    }
}

pub fn render_unthreaded(scene: &Arc<Scene>) {
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
