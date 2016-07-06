#![feature(collections, alloc)]
#![no_std]


extern crate alloc;
extern crate collections;
extern crate event_emitter;


use alloc::rc::Rc;
use core::cell::RefCell;
use core::any::Any;

use event_emitter::{EventEmitter, Emitter};


#[test]
fn test_event_emitter() {
    let mut emitter = EventEmitter::new();

    let on_test = |value: &Any| {
        match value.downcast_ref::<usize>() {
            Some(value) => assert!(*value == 1),
            None => panic!("value is not an usize"),
        }
    };

    let test_id = emitter.on("test", on_test);
    emitter.emit("test", &1usize);
    emitter.off("test", test_id);
    assert!(emitter.count("test") == 0);

    let x = Rc::new(RefCell::new(0));

    let ref_x = x.clone();
    emitter.on("test_mut", move |_| {
        *ref_x.borrow_mut() += 1;
    });

    emitter.emit("test_mut", &());
    emitter.emit("test_mut", &());
    emitter.emit("test_mut", &());

    assert!(*x.borrow() == 3);
}
