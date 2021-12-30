

struct Store {
    states: HashMap<Path, State>,
}

trait Anything: Any + Send {}

#[derive(Default, Clone)]
struct State {
    registers: Vec<Rc<dyn Anything>>,
    // whether this state got used the first time and new_state is allowed to allocate new
    // registers
    new: bool,
}

