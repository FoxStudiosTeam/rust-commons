#![feature(downcast_unchecked)]
#![feature(allocator_api)]

use std::any::{Any, TypeId};
use std::ops::Deref;

use anyhow::Result;
use axum::*;

use rust_commons;

#[derive(Default)]
struct DB {
    inner: Vec<String>
}

trait IDB {
    fn get(&self) -> &Vec<String>;
    fn push(&mut self, value: String);
}

impl IDB for DB {
    fn get(&self) -> &Vec<String> {
        &self.inner
    }
    fn push(&mut self, value: String) {
        self.inner.push(value);
    }
}


use rust_commons::di::iface::IContainer;

trait ProjectContainer : IContainer {
    fn get_db(&self) -> &dyn IDB;
}



#[tokio::main]
async fn main () -> Result<()> 
{
    let inner = Box::new(DB::default());
    let metadata = TypeId::of::<Box<dyn IDB>>();
    let inner: Box<dyn Any> = inner;

    let recovered: Box<DB> = unsafe {
        if inner.type_id() == TypeId::of::<DB>() {
            let raw: *mut dyn Any = Box::into_raw(inner);
            let raw = raw as *mut DB;
            Box::from_raw(raw)
        } else {
            panic!("type mismatch")
        }
    };

    Ok(())
}

// unsafe fn any_box_to_idb_unchecked<T + 'static>(b: Box<dyn Any>) -> Box<dyn T> {
//     let raw_any: *mut dyn Any = Box::into_raw(b);
//     // debug_assert!((&*raw_any).is::<T>());
//     let data_as_t: *mut T = raw_any.cast();
//     let boxed_t: Box<T> = Box::from_raw(data_as_t);
//     boxed_t
// }


