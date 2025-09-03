use std::any::Any;
use hashbrown::HashMap;

pub trait IDependencyBuilder {
    fn register_dep(&mut self,token: &str, dep: Box<dyn Any>) -> &mut Self;
    fn register_default(&mut self) -> &mut Self;
    fn build(self) -> HashMap<String, Box<dyn Any>>;
}