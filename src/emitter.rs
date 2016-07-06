use core::any::Any;


pub trait Emitter {
    fn on<F: Fn(&Any) + 'static>(&mut self, name: &str, func: F) -> usize;
    fn off(&mut self, name: &str, id: usize);
    fn emit(&mut self, name: &str, data: &Any);
}
