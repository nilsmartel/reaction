use crate::{
    component::{Component, ShapeRenderer, StateRenderer},
    context::Context,
    hooks::{Hooks, RenderHooks},
    key::{Key, Path},
    shape::Shape,
};

/// Concrete sample implementation of a component which has child components
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

    fn build(self: Box<Div>, ctx: Context) -> Box<dyn StateRenderer> {
        let items: Vec<Path> = self
            .items
            .into_iter()
            .map(|component| ctx.keep(component))
            .collect();

        Box::new(DivStateRenderer { items })
    }
}

struct DivStateRenderer {
    items: Vec<Path>,
}

impl StateRenderer for DivStateRenderer {
    fn compute(&self, _hooks: Hooks) -> Box<dyn ShapeRenderer> {
        Box::new(DivShapeRenderer {
            items: self.items.clone(),
        })
    }
}

struct DivShapeRenderer {
    items: Vec<Path>,
}

impl ShapeRenderer for DivShapeRenderer {
    fn render(&self, hooks: RenderHooks) -> Vec<Shape> {
        // TODO hooks.use_parent_size();

        // self.items.iter().map(hooks.use_shape);
        vec![]
    }
}
