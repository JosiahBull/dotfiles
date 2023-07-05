use singleton_derive::Singleton;

#[derive(Singleton)]
struct MyStruct {
    name: &'static str,
}

impl Default for MyStruct {
    fn default() -> Self {
        Self { name: "" }
    }
}

#[test]
fn test_singleton_new_returns_same_instance() {
    let instance1 = MyStruct::singleton();
    let instance2 = MyStruct::singleton();
    assert_eq!(instance1 as *const _, instance2 as *const _);
}

#[test]
fn test_singleton_fields_initialized_correctly() {
    let instance = MyStruct::singleton();
    assert_eq!(instance.name, "");
}

#[test]
fn test_singleton_default_impl() {
    let instance = MyStruct::default();
    assert_eq!(instance.name, "");
}

#[test]
fn test_singleton_static_initialized_correctly() {
    let instance = MyStruct::singleton();
    assert_eq!(instance.name, "");
}

#[test]
fn test_singleton_static_initialized_only_once() {
    let instance1 = MyStruct::singleton();
    let instance2 = MyStruct::singleton();
    assert_eq!(instance1 as *const _, instance2 as *const _);
}
