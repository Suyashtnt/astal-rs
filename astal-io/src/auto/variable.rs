// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-astal
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,VariableBase};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalIOVariable")]
    pub struct Variable(Object<ffi::AstalIOVariable, ffi::AstalIOVariableClass>) @extends VariableBase;

    match fn {
        type_ => || ffi::astal_io_variable_get_type(),
    }
}

impl Variable {
        pub const NONE: Option<&'static Variable> = None;
    

    #[doc(alias = "astal_io_variable_new")]
    pub fn new(init: &mut glib::Value) -> Variable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_io_variable_new(init.to_glib_none_mut().0))
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Variable`] objects.
            ///
            /// This method returns an instance of [`VariableBuilder`](crate::builders::VariableBuilder) which can be used to create [`Variable`] objects.
            pub fn builder() -> VariableBuilder {
                VariableBuilder::new()
            }
        
}

impl Default for Variable {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Variable`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct VariableBuilder {
            builder: glib::object::ObjectBuilder<'static, Variable>,
        }

        impl VariableBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn value(self, value: &glib::Value) -> Self {
                            Self { builder: self.builder.property("value", value.clone()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Variable`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Variable {
assert_initialized_main_thread!();
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Variable>> Sealed for T {}
}

pub trait VariableExt: IsA<Variable> + sealed::Sealed + 'static {
    #[doc(alias = "astal_io_variable_poll")]
    fn poll(&self, interval: u32, exec: &str, transform: Option<&glib::Closure>) -> Result<Variable, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_io_variable_poll(self.as_ref().to_glib_none().0, interval, exec.to_glib_none().0, transform.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_variable_pollv")]
    fn pollv(&self, interval: u32, execv: &[&str], transform: Option<&glib::Closure>) -> Result<Variable, glib::Error> {
        let execv_length1 = execv.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_io_variable_pollv(self.as_ref().to_glib_none().0, interval, execv.to_glib_none().0, execv_length1, transform.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_variable_pollfn")]
    fn pollfn(&self, interval: u32, fn_: &glib::Closure) -> Result<Variable, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_io_variable_pollfn(self.as_ref().to_glib_none().0, interval, fn_.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_variable_watch")]
    fn watch(&self, exec: &str, transform: Option<&glib::Closure>) -> Result<Variable, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_io_variable_watch(self.as_ref().to_glib_none().0, exec.to_glib_none().0, transform.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_variable_watchv")]
    fn watchv(&self, execv: &[&str], transform: Option<&glib::Closure>) -> Result<Variable, glib::Error> {
        let execv_length1 = execv.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_io_variable_watchv(self.as_ref().to_glib_none().0, execv.to_glib_none().0, execv_length1, transform.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_variable_start_poll")]
    fn start_poll(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_io_variable_start_poll(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_variable_start_watch")]
    fn start_watch(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_io_variable_start_watch(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_variable_stop_poll")]
    fn stop_poll(&self) {
        unsafe {
            ffi::astal_io_variable_stop_poll(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "astal_io_variable_stop_watch")]
    fn stop_watch(&self) {
        unsafe {
            ffi::astal_io_variable_stop_watch(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "astal_io_variable_is_polling")]
    fn is_polling(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_io_variable_is_polling(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_io_variable_is_watching")]
    fn is_watching(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_io_variable_is_watching(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_io_variable_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> glib::Value {
        unsafe {
            let mut result = glib::Value::uninitialized();
            ffi::astal_io_variable_get_value(self.as_ref().to_glib_none().0, result.to_glib_none_mut().0);
            result
        }
    }

    #[doc(alias = "astal_io_variable_set_value")]
    fn set_value(&self, value: &mut glib::Value) {
        unsafe {
            ffi::astal_io_variable_set_value(self.as_ref().to_glib_none().0, value.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P: IsA<Variable>, F: Fn(&P) + 'static>(this: *mut ffi::AstalIOVariable, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Variable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_value_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Variable>> VariableExt for O {}
