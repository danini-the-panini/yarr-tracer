use std::sync::Arc;

use scene::Scene;
use thread_pool::ThreadPool;

mod color;
mod interval;
mod math;
mod ray;
mod scene;
mod test_data;
mod thread_pool;
mod util;

fn main() {
    let scene = Arc::new(Scene::new(400, 225));

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
