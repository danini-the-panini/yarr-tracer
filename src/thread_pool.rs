use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::scene::Scene;

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub p: (usize, usize),
    pub color: (u8, u8, u8),
}

unsafe impl Send for Pixel {}
unsafe impl Sync for Pixel {}

pub fn render_threaded(size: usize, scene: &Scene) {
    let (tx, rx) = mpsc::channel::<(usize, usize)>();
    let (result_tx, result_rx) = mpsc::channel::<Pixel>();
    let rx = Arc::new(Mutex::new(rx));
    let result_tx = Arc::new(Mutex::new(result_tx));

    let mut image: Vec<(u8, u8, u8)> =
        vec![(0, 0, 0); scene.camera.image_width * scene.camera.image_height];

    scene.write_image_header();

    thread::scope(|s| {
        for _ in 0..size {
            let rx = Arc::clone(&rx);
            let result_tx = Arc::clone(&result_tx);
            s.spawn(move || {
                while let Ok((i, j)) = rx.lock().unwrap().recv() {
                    let color = scene.render(i, j);

                    result_tx
                        .lock()
                        .unwrap()
                        .send(Pixel { p: (i, j), color })
                        .expect("Failed to send pixel");
                }
            });
        }

        for j in 0..scene.camera.image_height {
            for i in 0..scene.camera.image_width {
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
            image[pixel.p.0 + pixel.p.1 * scene.camera.image_width] = pixel.color;
        }

        drop(result_rx);
    });

    eprintln!("\rDone.                   ");

    for color in image {
        scene.write_pixel(color);
    }
}

pub fn render_unthreaded(scene: &Scene) {
    scene.write_image_header();

    for j in 0..scene.camera.image_height {
        for i in 0..scene.camera.image_width {
            eprint!(
                "\rProgress: {}% ",
                (((j as f64) / (scene.camera.image_height as f64)) * 100.0) as u8
            );
            scene.write_pixel(scene.render(i, j));
        }
    }

    eprintln!("\rDone.                   ");
}
