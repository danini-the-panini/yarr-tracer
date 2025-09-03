use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

#[derive(Debug, Copy, Clone)]
struct Pixel {
    p: (usize, usize),
    color: (u8, u8, u8),
}

unsafe impl Send for Pixel {}
unsafe impl Sync for Pixel {}

#[derive(Debug, Copy, Clone)]
struct Scene {
    image_width: usize,
    image_height: usize,
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}

struct ThreadPool {
    tx: Sender<(usize, usize)>,
    rx: Receiver<Pixel>,
    handles: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize, scene: &Arc<Scene>) -> Self {
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
                    let r = (i as f64) / ((scene.image_width - 1) as f64);
                    let g = (j as f64) / ((scene.image_height - 1) as f64);
                    let b = 0.0;

                    let ir = (255.999 * r) as u8;
                    let ig = (255.999 * g) as u8;
                    let ib = (255.999 * b) as u8;

                    result_tx
                        .lock()
                        .unwrap()
                        .send(Pixel {
                            p: (i, j),
                            color: (ir, ig, ib),
                        })
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

fn main() {
    let scene = Arc::new(Scene {
        image_width: 256,
        image_height: 256,
    });

    let cpus = num_cpus::get();
    let pool = ThreadPool::new(cpus, &scene);

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
