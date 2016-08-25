use collections::string::String;
use collections::boxed::Box;
use collections::vec::Vec;
use collections::btree_map::BTreeMap;
use core::any::Any;

use emitter::Emitter;


pub struct EventEmitter {
    events: BTreeMap<String, Vec<Box<Fn(&Any)>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        EventEmitter {
            events: BTreeMap::new(),
        }
    }
    pub fn count(&self, name: &str) -> usize {
        match self.events.get(name) {
            Some(funcs) => funcs.len(),
            None => 0,
        }
    }
}

impl Emitter for EventEmitter {
    fn on<F: Fn(&Any) + 'static>(&mut self, name: &str, func: F) -> usize {
        let ref mut events = self.events;

        if !events.contains_key(name) {
            events.insert(String::from(name), Vec::new());
        }
        let funcs = events.get_mut(name).unwrap();
        let index = funcs.len();
        let f = Box::new(func) as Box<Fn(&Any)>;
        funcs.push(f);
        let id = (&funcs[index] as *const _) as usize;
        id
    }
    fn off(&mut self, name: &str, id: usize) {
        let funcs = self.events.get_mut(name).unwrap();

        match funcs.iter().position(|f| -> bool {
            let f_id = (f as *const _) as usize;
            f_id == id
        }) {
            Some(index) => {
                funcs.remove(index);
            },
            None => (),
        }
    }
    fn emit(&self, name: &str, data: &Any) {
        if let Some(funcs) = self.events.get(name) {
            for func in funcs.iter() {
                func(data);
            }
        }
    }
}
