use std::any::Any;
use std::collections::HashMap;

trait TestService{
    fn some_told(&self) -> String;
}

struct DualServiceTestStruct {
    value : String
}

impl DualServiceTestStruct {
    fn new (value: String) -> Self {
        Self{
            value
        }
    }
}

impl TestService for DualServiceTestStruct {
    fn some_told(&self) -> String {
        self.value.clone()
    }
}

struct TestServiceStruct {
    value : String
}

impl TestServiceStruct {
    fn new(value : String) -> Self {
        Self {
            value,
        }
    }
}

impl TestService for TestServiceStruct {
    fn some_told(&self) -> String {
        self.value.clone()
    }
}

trait SomeService {
    fn some_service_test(&self) -> String;
}

struct SomeServiceStruct{
    value : String
}

impl SomeServiceStruct {
    fn new(value: String) -> Self {
        Self {
            value
        }
    }
}

impl SomeService for SomeServiceStruct {
    fn some_service_test(&self) -> String {
        self.value.clone()
    }
}

trait ProjectContainer {
    fn get_test_service(&self) -> &Box<dyn TestService>;
    fn get_test_service_b(&self) -> &Box<dyn TestService>;

    fn get_some_service(&self) -> &Box<dyn SomeService>;
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

impl ProjectContainer for Container {
    fn get_test_service(&self) -> &Box<dyn TestService> {
        let res = self.deps.get("test");
        if let Some(rs) = res {
            if let Some(rr) = rs.downcast_ref::<Box<dyn TestService>>() {
                return rr
            }
            panic!("Не удалось закастить тип dyn Any к dyn TestService")
        }
        panic!("Неверный тип для зависимости test")
    }

    fn get_test_service_b(&self) -> &Box<dyn TestService> {
        let res = self.deps.get("test-b");
        if let Some(rs) = res {
            if let Some(rr) = rs.downcast_ref::<Box<dyn TestService>>() {
                return rr
            }
            panic!("Не удалось закастить тип dyn Any к dyn TestService")
        }
        panic!("Неверный тип для зависимости test-b")
    }
    
    fn get_some_service(&self) -> &Box<dyn SomeService> {
        let res = self.deps.get("some-service");
        if let Some(rs) = res {
            if let Some(rr) = rs.downcast_ref::<Box<dyn SomeService>>() {
                return rr
            }
            panic!("Не удалось закастить тип dyn Any к dyn SomeService")
        }
        panic!("Нет зависимости в di test_service_b")
    }
}

#[cfg(test)]
mod tests {
    use crate::di::impls::DependencyBuilder;
    use super::*;

    #[test]
    fn test_di() {
        let test_service = TestServiceStruct::new("Some-value-a".to_string());
        let test_service_b = DualServiceTestStruct::new("Some-value-b".to_string());
        let some_service = SomeServiceStruct::new("Some-service-value".to_string());
        let mut builder = DependencyBuilder::new();
        builder.register_dep("test", Box::new(Box::new(test_service) as Box<dyn TestService>));
        builder.register_dep("test-b", Box::new(Box::new(test_service_b) as Box<dyn TestService>));
        builder.register_dep("some-service", Box::new(Box::new(some_service) as Box<dyn SomeService>));
        let deps = builder.build();
        let di = Container::new(deps);
        
        assert_eq!("Some-value-a".to_string(), di.get_test_service().some_told());
        assert_eq!("Some-value-b".to_string(), di.get_test_service_b().some_told());
        assert_eq!("Some-service-value".to_string(), di.get_some_service().some_service_test());
        assert_eq!(3, di.deps.len())
    }
}