mod di;

use crate::di::iface::IContainer;
use crate::di::impls::Container;

use std;
use std::any::Any;

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

trait ProjectContainer : IContainer {
    fn get_test_service(&self) -> &TestServiceStruct;
    fn get_test_service_b(&self) -> &dyn TestService;
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
}

#[cfg(test)]
mod tests {
    use crate::di::impls::DependencyBuilder;
    use super::*;

    #[test]
    fn test_di() {
        let test_service = TestServiceStruct::new("Some-value-a".to_string());
        let test_service_b = TestServiceStruct::new("Some-value-b".to_string());
        let mut builder = DependencyBuilder::new();
        builder.register_dep("test", Box::new(test_service));
        builder.register_dep("test-b", Box::new(test_service_b));
        let deps = builder.build();
        let mut di = Container::new(deps);

        assert_eq!("Some-value-a".to_string(), di.get_test_service().some_told());
        assert_eq!("Some-value-b".to_string(), di.get_test_service_b().some_told());
        assert_eq!(2, di.deps.len())
    }
}