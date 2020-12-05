use core::u64;
use core::intrinsics::transmute;
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)] pub struct Value {
    _private: [u8; 0]
}

#[repr(C)] pub struct Executable {
    _private: [u8; 0]
}

#[repr(C)] pub struct Constructor {
    _private: [u8; 0]
}

extern "C" {
    fn polyglot_import(name: *const c_char) -> *mut Value;
    fn polyglot_export(name: *const c_char, value: *const Value);
    fn polyglot_java_type(classname: *const c_char) -> *mut Value;
    fn polyglot_is_null(value: *const Value) -> bool;
    fn polyglot_can_execute(value: *const Value) -> bool;
    fn polyglot_can_instantiate(value: *const Value) -> bool;
    fn polyglot_from_string_n(string: *const u8, size: u64, charset: *const u8) -> *mut Value;
    fn polyglot_is_string(value: *const Value) -> bool;
    fn polyglot_get_string_size(value: *const Value) -> u64;
    fn polyglot_fits_in_i32(value: *const Value) -> bool;
    fn polyglot_as_i32(value: *const Value) -> i32;
    fn polyglot_is_boolean(value: *const Value) -> bool;
    fn polyglot_as_boolean(value: *const Value) -> bool;
}

pub mod internal {
    extern "C" {
        pub fn polyglot_invoke(value: *mut super::Value, name: *const super::c_char, ...) -> *mut super::Value;
        pub fn polyglot_new_instance(value: *const super::Constructor, ...) -> *mut super::Value;
    }

    pub trait PolyglotVariadic {}

    impl PolyglotVariadic for *const super::Value {}
    impl PolyglotVariadic for *mut super::Value {}
    impl PolyglotVariadic for i8 {}
    impl PolyglotVariadic for i16 {}
    impl PolyglotVariadic for i32 {}
    impl PolyglotVariadic for i64 {}

    pub fn expect_variadic<T : PolyglotVariadic>(value: T) -> T {
        value
    }

    pub fn make_cstring(string: &str) -> super::CString {
        super::CString::new(string).expect("Could not convert to CString")
    }

    pub fn transmute_executable(executable: *const super::Executable) -> extern "C" fn(*const super::Value, ...) -> *mut super::Value {
        unsafe {
            super::transmute(executable)
        }
    }

    pub fn transmute_executable_nullary(executable: *const super::Executable) -> extern "C" fn() -> *mut super::Value {
        unsafe {
            super::transmute(executable)
        }
    }
}

#[macro_export]
macro_rules! new_instance {
    ($constructor: expr) => {{
        unsafe {
            $crate::polyglot::internal::polyglot_new_instance(
                $constructor
            )
        }
    }};
    ($constructor: expr, $($args: expr),*) => {{
        unsafe {
            $crate::polyglot::internal::polyglot_new_instance(
                $constructor,
                $($crate::polyglot::internal::expect_variadic($args)),*
            )
        }
    }}
}

#[macro_export]
macro_rules! invoke_method {
    ($value: expr, $method: expr) => {{
        unsafe {
            $crate::polyglot::internal::polyglot_invoke(
                $value,
                $crate::polyglot::internal::make_cstring($method).as_ptr()
            )
        }
    }};
    ($value: expr, $method: expr, $($args: expr),+) => {{
        unsafe {
            $crate::polyglot::internal::polyglot_invoke(
                $value,
                $crate::polyglot::internal::make_cstring($method).as_ptr(),
                $($crate::polyglot::internal::expect_variadic($args)),*
            )
        }
    }}
}

#[macro_export]
macro_rules! execute {
    ($executable: expr) => {{
        let fnptr = $crate::polyglot::internal::transmute_executable_nullary($executable);
        fnptr()
    }};
    ($executable: expr, $($args: expr),+) => {{
        let fnptr = $crate::polyglot::internal::transmute_executable($executable);
        fnptr($($crate::polyglot::internal::expect_variadic($args)),+)
    }};
}

pub fn from_string(str: &str) -> *mut Value {
    if str.len() > u64::MAX as usize {
        panic!("String is too long");
    }
    let ptr = str.as_ptr();
    let charset = "UTF-8\0".as_ptr();
    let len: u64 = str.len() as u64;
    unsafe {
        polyglot_from_string_n(ptr, len, charset)
    }
}

pub fn is_string(value: *const Value) -> bool {
    unsafe { polyglot_is_string(value) }
}

pub fn get_string_size(value: *const Value) -> u64 {
    if !is_string(value) {
        panic!("Not a string")
    };
    unsafe {
        polyglot_get_string_size(value)
    }
}

pub fn is_null(value: *const Value) -> bool {
    unsafe { polyglot_is_null(value) }
}

pub fn import(name: &str) -> *mut Value {
    let c_str = internal::make_cstring(name);
    let value = unsafe { polyglot_import(c_str.as_ptr()) };
    if is_null(value) {
        panic!("Import failed")
    }
    value
}

pub fn export(name: &str, value: *mut Value) {
    let c_str = internal::make_cstring(name);
    unsafe { polyglot_export(c_str.as_ptr(), value) }
}

pub fn as_executable(value: *const Value) -> *const Executable {
    unsafe {
        if !polyglot_can_execute(value) {
            panic!("Value is not executable")
        }
        transmute(value)
    }
}

pub fn java_type(name: &str) -> *mut Constructor {
    let c_str = internal::make_cstring(name);
    let value = unsafe { polyglot_java_type(c_str.as_ptr()) };
    if is_null(value) || !unsafe { polyglot_can_instantiate(value) } {
        panic!("Not a type")
    }
    unsafe { transmute(value) }
}

pub fn as_i32(value: *const Value) -> i32 {
    unsafe {
        if !polyglot_fits_in_i32(value) {
            panic!("Not a number")
        }
        polyglot_as_i32(value)
    }
}

pub fn as_boolean(value: *const Value) -> bool {
    unsafe {
        if !polyglot_is_boolean(value) {
            panic!("Not a boolean")
        }
        polyglot_as_boolean(value)
    }
}