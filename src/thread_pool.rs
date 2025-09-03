use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use crate::{color::Color, ray::Ray, rgb, scene::Scene};

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

fn ray_color(r: &Ray) -> Color {
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
