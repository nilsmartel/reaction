use std::rc::Rc;

use crate::{
    component::Component,
    context::Context,
    key::Key,
    renderable::{Renderable, Shape},
};

struct Div {
    key: Option<Key>,
    items: Vec<Box<dyn Component>>,
}

impl Component for Div {
    fn get_key(&self) -> Option<&Key> {
        if let Some(ref key) = self.key {
            return Some(&key);
        }

        None
    }

    fn set_key(&mut self, key: Key) {
        self.key = Some(key);
    }

    fn build(&self, ctx: &Context) -> &dyn Renderable {
        let renderables: Vec<_> = self
            .items
            .iter()
            .map(|component| ctx.associate(component.as_ref()))
            .collect();

        let renderables = Rc::new(renderables);
        let r = renderables.clone();

        &Renderer::new(move || {
            // TODO this should be more complicated in practice
            // This is just stacking all elements on top of each other
            (&r).iter().map(|r| r.render()).flatten().collect()
        })
    }
}

struct Renderer<T: Fn() -> Vec<Shape>> {
    render_fn: T,
}

impl<T: Fn() -> Vec<Shape>> Renderer<T> {
    fn new(render_fn: T) -> Self {
        Self { render_fn }
    }
}

impl<T> Renderable for Renderer<T>
where
    T: Fn() -> Vec<Shape>,
{
    fn render(&self) -> Vec<Shape> {
        let r = &self.render_fn;
        r()
    }
}
