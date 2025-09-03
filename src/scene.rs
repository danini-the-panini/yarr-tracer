#[derive(Debug, Copy, Clone)]
pub struct Scene {
    pub image_width: usize,
    pub image_height: usize,
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}
