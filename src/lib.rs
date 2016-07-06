#![feature(collections)]
#![no_std]


extern crate collections;


mod emitter;
mod event_emitter;


pub use emitter::Emitter;
pub use event_emitter::EventEmitter;
