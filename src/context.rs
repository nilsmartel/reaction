use crate::{component::*, key::Key};
use std::{any::Any, collections::HashMap, rc::Rc, sync::Mutex};

pub type Path = Vec<Key>;

struct Runtime {
    state_renderers: HashMap<Path, Box<dyn StateRenderer>>,
}

pub struct Context {
    // TODO actually needs no mutable store
    // but ability to push new state_renderers
    runtime: &'static Mutex<Runtime>,
    // TODO needs ot state, this is what Hooks are for
    child_id_counter: Mutex<isize>,
    path: Path,
}

impl Context {
    fn next_key(&self) -> isize {
        let mut counter = self.child_id_counter.lock().expect("to use key counter");
        let key = *counter;
        *counter += 1;
        key
    }

    pub fn keep(&self, component: Box<dyn Component>) -> Path {
        // get key for component attempt to generate unique one.
        // Note that the generated key is not guaranteed to be unique and the application might
        // crash subsequently.
        let key = component
            .get_key()
            .map(|k| k.clone())
            .unwrap_or_else(|| self.next_key().into());

        // generate unique path for this element.
        let mut path = Vec::with_capacity(self.path.len() + 1);
        path.push(key);

        // generate new state to associate with component
        let ctx = Context {
            path,
            child_id_counter: Mutex::new(0),
            runtime: self.runtime,
        };

        // Extract state rendererer
        let state_renderer = Box::new(component).build(ctx);

        let mut rt = self
            .runtime
            .lock()
            .expect("to update state_renderer in runtime");
        rt.state_renderers[&path] = state_renderer;

        path
    }
}

/*
        // TODO these next few lines are actually logic for hooks

        // Fetch the state of the child
        let state = {
            // Make new state known or insert fresh one for this component
            let mut store = self.store.lock().expect("to lock global state");
            store
                .states
                .entry(path.clone())
                // make sure the state is no longer fresh, if it's fetched from the state database
                .and_modify(|s| s.new = false)
                .or_insert(State::default())
                .clone()
        };

// Fetch the state of the child
        let state = {
            let mut store = self.store.lock().expect("to lock global state");
            store
                .states
                .entry(path.clone())
                .or_insert(State::default())
                .clone()
        };
*/
