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
    fn polyglot_java_type(classname: *const c_char) -> *mut Value;
    fn polyglot_is_null(value: *const Value) -> bool;
    pub fn polyglot_invoke(value: *mut Value, name: *const u8, ...) -> *mut Value;
    fn polyglot_can_execute(value: *const Value) -> bool;
    fn polyglot_can_instantiate(value: *const Value) -> bool;
    pub fn polyglot_new_instance(value: *const Constructor, ...) -> *mut Value;
    fn polyglot_from_string_n(string: *const u8, size: u64, charset: *const u8) -> *mut Value;
    fn polyglot_is_string(value: *const Value) -> bool;
    fn polyglot_get_string_size(value: *const Value) -> u64;
    fn polyglot_fits_in_i32(value: *const Value) -> bool;
    fn polyglot_as_i32(value: *const Value) -> i32;
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

fn make_cstring(string: &str) -> CString {
    CString::new(string).expect("Could not convert to CString")
}

pub fn import(name: &str) -> *mut Value {
    let c_str = make_cstring(name);
    let value = unsafe { polyglot_import(c_str.as_ptr()) };
    if is_null(value) {
        panic!("Import failed")
    }
    value
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
    let c_str = make_cstring(name);
    let value = unsafe { polyglot_java_type(c_str.as_ptr()) };
    if is_null(value) || !unsafe { polyglot_can_instantiate(value) } {
        panic!("Not a type")
    }
    unsafe { transmute(value) }
}

pub fn invoke(executable: *const Executable, arg1: *const Value) -> *mut Value {
    let fnptr: extern "C" fn(*const Value) -> *mut Value = unsafe { transmute(executable) };
    fnptr(arg1)
}

pub fn as_i32(value: *const Value) -> i32  {
    unsafe {
        if !polyglot_fits_in_i32(value) {
            panic!("Not a number")
        }
        polyglot_as_i32(value)
    }
}
