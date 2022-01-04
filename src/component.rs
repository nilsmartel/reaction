use crate::context::*;
use crate::key::*;
use crate::shape::*;
use crate::hooks::*;

pub trait Component {
    fn get_key(&self) -> Option<&Key>;

    fn set_key(&mut self, key: Key);

    fn build(self: Box<Self>, ctx: Context) -> Box<dyn StateRenderer>;
}

pub trait StateRenderer {
    fn compute(&self, hooks: Hooks) -> Box<dyn ShapeRenderer>;
}

pub trait ShapeRenderer {
    // TODO return ((width, height), Vec<Shape>)
    fn render(&self, hooks: RenderHooks) -> Vec<Shape>;
}

