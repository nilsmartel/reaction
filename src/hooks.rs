use crate::key::Path;
use crate::shape::Shape;

pub struct Hooks {}

pub struct RenderHooks {}

impl RenderHooks {
    // Id is the id of some shape renderer
    fn use_shape(&self, id: Path) -> Vec<Shape> {
        unimplemented!()
    }
}
