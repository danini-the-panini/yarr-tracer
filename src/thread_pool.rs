use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::scene::Scene;

pub fn render_threaded(size: usize, scene: &Scene) {
    eprintln!("RUNNING ON {} CPUS", size);
    let (tx, rx) = mpsc::channel::<usize>();
    let (result_tx, result_rx) = mpsc::channel::<(usize, Vec<(u8, u8, u8)>)>();
    let rx = Arc::new(Mutex::new(rx));
    let result_tx = Arc::new(Mutex::new(result_tx));

    let mut image: Vec<(u8, u8, u8)> =
        vec![(0, 0, 0); scene.camera.image_width * scene.camera.image_height];

    scene.write_image_header();

    thread::scope(|s| {
        for _ in 0..size {
            let rx = Arc::clone(&rx);
            let result_tx = Arc::clone(&result_tx);
            s.spawn(move || loop {
                let msg = rx.lock().unwrap().recv();

                match msg {
                    Ok(j) => {
                        let row: Vec<(u8, u8, u8)> = (0..scene.camera.image_width)
                            .into_iter()
                            .map(|i| scene.render(i, j))
                            .collect();

                        result_tx
                            .lock()
                            .unwrap()
                            .send((j, row))
                            .expect("Failed to send pixel");
                    }
                    Err(_) => {
                        break;
                    }
                }
            });
        }

        for j in 0..scene.camera.image_height {
            tx.send(j).expect("Failed to send pixel");
        }

        drop(tx);

        for j in 0..scene.camera.image_height {
            eprint!(
                "\rProgress: {}% ",
                (((j as f64) / (scene.camera.image_height as f64)) * 100.0) as u8
            );
            let (j, row) = result_rx.recv().expect("Failed to receive pixel");
            let row_start = j * scene.camera.image_width;
            image[row_start..row_start + scene.camera.image_width].copy_from_slice(&row);
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
