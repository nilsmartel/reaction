use crate::{
    component::*,
    key::Path,
    runtime::*,
};
use std::sync::Mutex;

/// uniquely identifies components in the component tree.
/// Manages their integration into the runtime, as well as the integration of sub-components.
pub struct Context {
    /// References the the current Path uniquely associated with the component, this Context got
    /// created for
    path: Path,

    /// Keeps track of id of sub-components (children)
    /// NOTE: would be nice to remove the Mutex around this.
    child_id_counter: Mutex<isize>,

    /// Reference to the runtime, used in order to integrate further sub-components
    runtime: &'static Mutex<Runtime>,

}

impl Context {
    /// generate next available id for a child.
    fn next_key(&self) -> isize {
        let mut counter = self.child_id_counter.lock().expect("to use key counter");
        let key = *counter;
        *counter += 1;
        key
    }

    /// Integrates a component into the runtime and returns a reference in form of it's distinct
    /// key to it.
    pub fn keep(&self, component: Box<dyn Component>) -> Path {
        // get key for component attempt to generate unique one.
        // Note that the generated key is not guaranteed to be unique and the application might
        // crash subsequently.
        let key = component
            .get_key()
            .map(|k| k.clone())
            .unwrap_or_else(|| self.next_key().into());

        // generate unique path for this element.
        let path = {
            let mut p = Vec::with_capacity(self.path.len() + 1);
            p.push(key);
            p
        };

        // generate new state to associate with component
        let ctx = Context {
            path: path.clone(),
            child_id_counter: Mutex::new(0),
            runtime: self.runtime,
        };

        // Extract state rendererer
        let state_renderer = Box::new(component).build(ctx);

        let mut rt = self
            .runtime
            .lock()
            .expect("to update state_renderer in runtime");

        rt.state_renderers.insert(path.clone(), state_renderer);

        path
    }
}
