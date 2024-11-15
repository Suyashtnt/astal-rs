// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-astal
// from ../gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalIOProcess")]
    pub struct Process(Object<ffi::AstalIOProcess, ffi::AstalIOProcessClass>);

    match fn {
        type_ => || ffi::astal_io_process_get_type(),
    }
}

impl Process {
        pub const NONE: Option<&'static Process> = None;
    

    #[doc(alias = "astal_io_process_new_subprocessv")]
    pub fn subprocessv(cmd: &[&str]) -> Result<Process, glib::Error> {
        assert_initialized_main_thread!();
        let cmd_length1 = cmd.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_io_process_new_subprocessv(cmd.to_glib_none().0, cmd_length1, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_process_new")]
    pub fn new() -> Process {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_io_process_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Process`] objects.
            ///
            /// This method returns an instance of [`ProcessBuilder`](crate::builders::ProcessBuilder) which can be used to create [`Process`] objects.
            pub fn builder() -> ProcessBuilder {
                ProcessBuilder::new()
            }
        

    #[doc(alias = "astal_io_process_subprocess")]
    pub fn subprocess(cmd: &str) -> Result<Process, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_io_process_subprocess(cmd.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_process_execv")]
    pub fn execv(cmd: &[&str]) -> Result<glib::GString, glib::Error> {
        assert_initialized_main_thread!();
        let cmd_length1 = cmd.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_io_process_execv(cmd.to_glib_none().0, cmd_length1, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_process_exec")]
    pub fn exec(cmd: &str) -> Result<glib::GString, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_io_process_exec(cmd.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //#[doc(alias = "astal_io_process_exec_asyncv")]
    //pub fn exec_asyncv(cmd: &[&str], _callback_: AsyncReadyCallback) {
    //    unsafe { TODO: call ffi:astal_io_process_exec_asyncv() }
    //}

    //#[doc(alias = "astal_io_process_exec_async")]
    //pub fn exec_async(cmd: &str, _callback_: AsyncReadyCallback) {
    //    unsafe { TODO: call ffi:astal_io_process_exec_async() }
    //}
}

impl Default for Process {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Process`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ProcessBuilder {
            builder: glib::object::ObjectBuilder<'static, Process>,
        }

        impl ProcessBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn argv(self, argv: impl Into<glib::StrV>) -> Self {
                            Self { builder: self.builder.property("argv", argv.into()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Process`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Process {
assert_initialized_main_thread!();
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Process>> Sealed for T {}
}

pub trait ProcessExt: IsA<Process> + sealed::Sealed + 'static {
    #[doc(alias = "astal_io_process_kill")]
    fn kill(&self) {
        unsafe {
            ffi::astal_io_process_kill(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "astal_io_process_signal")]
    fn signal(&self, signal_num: i32) {
        unsafe {
            ffi::astal_io_process_signal(self.as_ref().to_glib_none().0, signal_num);
        }
    }

    #[doc(alias = "astal_io_process_write")]
    fn write(&self, in_: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_io_process_write(self.as_ref().to_glib_none().0, in_.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //#[doc(alias = "astal_io_process_write_async")]
    //fn write_async(&self, in_: &str, _callback_: AsyncReadyCallback) {
    //    unsafe { TODO: call ffi:astal_io_process_write_async() }
    //}

    #[doc(alias = "astal_io_process_get_argv")]
    #[doc(alias = "get_argv")]
    fn argv(&self) -> Vec<glib::GString> {
        unsafe {
            let mut result_length1 = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(ffi::astal_io_process_get_argv(self.as_ref().to_glib_none().0, result_length1.as_mut_ptr()), result_length1.assume_init() as _);
            ret
        }
    }

    #[doc(alias = "stdout")]
    fn connect_stdout<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stdout_trampoline<P: IsA<Process>, F: Fn(&P, &str) + 'static>(this: *mut ffi::AstalIOProcess, out: *const std::ffi::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Process::from_glib_borrow(this).unsafe_cast_ref(), &glib::GString::from_glib_borrow(out))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"stdout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(stdout_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "stderr")]
    fn connect_stderr<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stderr_trampoline<P: IsA<Process>, F: Fn(&P, &str) + 'static>(this: *mut ffi::AstalIOProcess, err: *const std::ffi::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Process::from_glib_borrow(this).unsafe_cast_ref(), &glib::GString::from_glib_borrow(err))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"stderr\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(stderr_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Process>> ProcessExt for O {}