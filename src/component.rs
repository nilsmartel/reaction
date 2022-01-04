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

fn component<C: 'static + Component>(f: impl Fn(Hooks) -> C + 'static) -> BlanketComponent {
    BlanketComponent {
        identifier: None,
        f: Box::new(|h: Hooks| Box::new(f(h))),
    }
}

struct BlanketComponent {
    identifier: Option<Key>,
    f: Box<dyn Fn(Hooks) -> Box<dyn Component>>,
}

impl Component for BlanketComponent {
    fn get_key(&self) -> Option<&Key> {
        if let Some(ref key) = self.identifier {
            Some(&key);
        }
        None
    }

    fn set_key(&mut self, key: Key) {
        self.identifier = Some(key);
    }

    fn build(self: Box<Self>, ctx: Context) -> Box<dyn StateRenderer> {
        Box::new(BlanketStateRenderer { ctx, f: self.f })
    }
}

struct BlanketStateRenderer {
    ctx: Context,
    f: Box<dyn Fn(Hooks) -> Box<dyn Component>>,
}

impl StateRenderer for BlanketStateRenderer {
    fn compute(&self, hooks: Hooks) -> Box<dyn ShapeRenderer> {
        let f = self.f;
        let c = f(hooks);
        let child_id = self.ctx.keep(c);
        Box::new(BlanketShapeRenderer { child_id })
    }
}

struct BlanketShapeRenderer {
    child_id: Path,
}

impl ShapeRenderer for BlanketShapeRenderer {
    fn render(&self, hooks: RenderHooks) -> Vec<Shape> {
        hooks.use_shape(self.child_id.clone())
    }
}
