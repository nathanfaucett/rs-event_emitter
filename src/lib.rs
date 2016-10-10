#![feature(collections)]
#![no_std]


extern crate collections;


mod emitter;
mod event_emitter;


pub use emitter::Emitter;
pub use event_emitter::EventEmitter;


#[macro_export]
macro_rules! impl_Emitter {
    ($e: ident) => (
        fn on<F: Fn(&Any) + 'static>(&mut self, name: &str, func: F) -> usize {
            self.$e.on(name, func)
        }
        fn off(&mut self, name: &str, id: usize) {
            self.$e.off(name, id);
        }
        fn emit(&self, name: &str, data: &Any) {
            self.$e.emit(name, data);
        }
    )
}
