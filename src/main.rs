use std::sync::Arc;

use scene::Scene;
use thread_pool::{render_threaded, render_unthreaded};

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

    render_threaded(cpus, &scene);
    // render_unthreaded(&scene);
}
