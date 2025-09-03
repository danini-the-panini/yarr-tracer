use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use crate::{color::Color, rgb, scene::Scene};

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

fn render(i: usize, j: usize, scene: &Arc<Scene>) -> (u8, u8, u8) {
    let pixel_color = rgb!(
        (i as f64) / ((scene.image_width - 1) as f64),
        (j as f64) / ((scene.image_height - 1) as f64),
        0.0
    );

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
