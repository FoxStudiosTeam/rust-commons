use std::any::Any;
use std::collections::HashMap;
use crate::di::iface::{IContainer, IDependencyBuilder};

pub struct DependencyBuilder {
    deps: HashMap<String, Box<dyn Any>>
}

pub enum Dependency<T>{
    Ok(T)
}

impl DependencyBuilder {
    pub fn new() -> Self {
        Self {
            deps: HashMap::new()
        }
    }
    pub fn register_default(&mut self) -> &mut Self {
        IDependencyBuilder::register_default(self)
    }
    pub fn register_dep(&mut self,token: &str, dep: Box<dyn Any>) -> &mut Self {
        IDependencyBuilder::register_dep(self, token, dep)
    }
    pub fn build(self) -> HashMap<String, Box<dyn Any>> {
        IDependencyBuilder::build(self)
    }
}

impl IDependencyBuilder for DependencyBuilder {

    fn register_default(&mut self) -> &mut Self {
        //todo self.registerDep("some name",Box::new(some lib::new()))
        return self
    }
    fn register_dep(&mut self,token: &str, dep: Box<dyn Any>) -> &mut Self {
        self.deps.insert(token.to_string(), dep);
        return self
    }
    fn build(self) -> HashMap<String, Box<dyn Any>> {
        self.deps
    }
}

#[derive(Default)]
pub struct Container {
    pub deps: HashMap<String, Box<dyn Any>>
}

impl Container {
    pub fn new(hash_map: HashMap<String, Box<dyn Any>>) -> Self {
        Self {
            deps: hash_map
        }
    }
}

impl IContainer for Container {
    //TODO название сервиса() -> тип сервиса
}
