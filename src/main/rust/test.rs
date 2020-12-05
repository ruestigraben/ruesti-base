#[cfg(feature = "ruesti_test")]
mod test {
    use crate::*;
    use crate::polyglot::*;

    fn consumer_unary() {
        let executable = as_executable(import("__test_consumer1"));
        execute!(executable, from_string("Hello from Rust"));
    }

    fn consumer_binary() {
        let executable = as_executable(import("__test_consumer2"));
        execute!(executable, from_string("Hello again"), 3i32);
    }

    fn runnable() {
        let executable = as_executable(import("__test_runnable"));
        execute!(executable);
    }

    fn create_list_nullary() {
        let constructor = java_type("java.util.ArrayList");
        let list = new_instance!(constructor);
        invoke_method!(list, "add", 2);
        export("__response_array1", list)
    }

    fn create_list() {
        let constructor = java_type("java.util.ArrayList");
        let list = new_instance!(constructor, 5);
        invoke_method!(list, "add", 12);
        invoke_method!(list, "add", from_string("yolo"));
        export("__response_array2", list);

        if !as_boolean(invoke_method!(list, "equals", list)) {
            panic!("Equals failed")
        }

        as_i32(invoke_method!(list, "hashCode"));
    }

    #[no_mangle]
    pub fn __test_success() {
        consumer_unary();
        consumer_binary();
        runnable();
        create_list_nullary();
        create_list();
    }

    #[no_mangle]
    pub fn __test_failure() {
        panic!("at the disco")
    }
}
