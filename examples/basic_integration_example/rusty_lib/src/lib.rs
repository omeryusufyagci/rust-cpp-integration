// Greeter interface
pub trait Greeter {
    fn say_hello(&self);
}

// Struct that will provide Greeter implementations
pub struct RustGreeter {
    name: String,
}

impl Greeter for RustGreeter {
    fn say_hello(&self) {
        println!("Hello from RustGreeter! My name is {}", self.name);
    }
}

#[no_mangle]
pub extern "C" fn generate_greeter(name: *const std::os::raw::c_char) -> *mut RustGreeter {
    if name.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { std::ffi::CStr::from_ptr(name) };
    let rust_string = match c_str.to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => return std::ptr::null_mut(),
    };

    let greeter = RustGreeter { name: rust_string };
    Box::into_raw(Box::new(greeter))
}

#[no_mangle]
pub extern "C" fn greeter_say_hello(greeter: *mut RustGreeter) {
    unsafe {
        if let Some(g) = greeter.as_ref() {
            g.say_hello();
        }
    }
}

#[no_mangle]
pub extern "C" fn destroy_greeter(greeter: *mut RustGreeter) {
    if !greeter.is_null() {
        unsafe {
            drop(Box::from_raw(greeter));
        }
    }
}
