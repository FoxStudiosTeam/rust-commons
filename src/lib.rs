mod di;

use crate::di::iface::IContainer;
use crate::di::impls::Container;

trait TestService{
    fn some_told(&self) -> String;
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

trait ProjectContainer : IContainer {
    fn get_test_service(&self) -> &TestServiceStruct;
    fn get_test_service_b(&self) -> &dyn TestService;

    fn get_some_service(&self) -> &dyn SomeService;
}

impl ProjectContainer for Container {
    fn get_test_service(&self) -> &TestServiceStruct {
        let res = self.deps.get("test");
        if let Some(test_service_ptr) = res {
            let a = test_service_ptr.downcast_ref::<TestServiceStruct>();
            if let Some(test_service) = a {
                return test_service
            }
        }
        panic!("Нет зависимости в di test_service")
    }

    fn get_test_service_b(&self) -> &dyn TestService {
        let res = self.deps.get("test-b");
        if let Some(test_service_ptr) = res {
            let a = test_service_ptr.downcast_ref::<TestServiceStruct>();
            if let Some(test_service) = a {
                return test_service
            }
        }
        panic!("Нет зависимости в di test_service_b")
    }

    fn get_some_service(&self) -> &dyn SomeService {
        let rs = self.deps.get("some-service");
        if let Some(some_service_heap_ptr) = rs {
            let ptr = some_service_heap_ptr.downcast_ref::<SomeServiceStruct>();
            if let Some(some_service_ptr) = ptr {
                return some_service_ptr
            }
        }
        panic!("Нет зависимости в di some_service")
    }
}

#[cfg(test)]
mod tests {
    use crate::di::impls::DependencyBuilder;
    use super::*;

    #[test]
    fn test_di() {
        let test_service = TestServiceStruct::new("Some-value-a".to_string());
        let test_service_b = TestServiceStruct::new("Some-value-b".to_string());
        let some_service = SomeServiceStruct::new("Some-service-value".to_string());
        let mut builder = DependencyBuilder::new();
        builder.register_dep("test", Box::new(test_service));
        builder.register_dep("test-b", Box::new(test_service_b));
        builder.register_dep("some-service", Box::new(some_service));
        let deps = builder.build();
        let mut di = Container::new(deps);

        assert_eq!("Some-value-a".to_string(), di.get_test_service().some_told());
        assert_eq!("Some-value-b".to_string(), di.get_test_service_b().some_told());
        assert_eq!("Some-service-value".to_string(), di.get_some_service().some_service_test());
        assert_eq!(3, di.deps.len())
    }
}