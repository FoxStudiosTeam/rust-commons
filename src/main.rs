mod di;

use std::any::Any;
use di::impls::{DependencyBuilder};

fn main() {
    let mut dep = DependencyBuilder::new();
    dep
        .register_default()
        .register_dep("123", Box::new("321")); // <- "321" - &str
    let map = dep.build();
    println!("{}", map.get("123").unwrap().downcast_ref::<&str>().unwrap()) // для получения объекта get("key).unwrap().downcast_ref::<тип получаемого объекта>().unwrap()
}