use crate::{
    component::{ShapeRenderer, StateRenderer},
    key::Path,
    state::Store,
};
use std::collections::HashMap;

pub struct Runtime {
    state_renderers: HashMap<Path, Box<dyn StateRenderer>>,
    shape_renderer: Box<dyn ShapeRenderer>,

    /// Storage for all state
    store: Store,
}

