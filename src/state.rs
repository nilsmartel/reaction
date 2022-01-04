use crate::key::Path;
use std::{any::Any, collections::HashMap, rc::Rc};

pub(crate) struct Store {
    states: HashMap<Path, State>,
}

trait Anything: Any + Send {}

#[derive(Default, Clone)]
struct State {
    /// Immutable reference to data behind in this state
    registers: Vec<Rc<dyn Any>>,
}

/// A helper to retrieve registers from a state in order
#[derive(Default, Clone)]
struct StateView {
    /// Readonly copy of State
    state: State,

    /// Currently accessed register
    index: usize,

    /// Path indexing this particular state
    path: Path,

    /// whether this state got used the first time
    /// and new_state is allowed to allocate new registers
    new: bool,
}

impl StateView {
    fn next_value<T: 'static + Sized>(&mut self) -> Result<&T, Error> {
        if self.index >= self.state.registers.len() {
            return Err(Error::Index(StateIndexError {
                message: "attempted to access more values, than state contains",
                path: self.path.clone(),
            }));
        }

        // Increment index, pointing to next state register
        let index = self.index;
        self.index += 1;

        let value = (self.state.registers[index]).downcast_ref::<T>();

        value.ok_or_else(|| {
            Error::Type(TypeCoerccionError {
                got_type: std::any::type_name::<T>(),
                message: "attempted to access state using invald type",
            })
        })
    }

    fn index(&self) -> usize {
        self.index
    }
}

pub enum Error {
    Index(StateIndexError),
    Type(TypeCoerccionError),
}

pub struct TypeCoerccionError {
    pub got_type: &'static str,
    pub message: &'static str,
}

pub struct StateIndexError {
    pub path: Path,
    pub message: &'static str,
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

