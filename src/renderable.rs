#[derive(Copy, Clone)]
pub enum Shape {
    Box(f32, f32, f32, f32),
}

pub trait Renderable {
    // TODO later on another context will be added in order to have hooks for rendering
    fn render(&self) -> Vec<Shape>;
}
