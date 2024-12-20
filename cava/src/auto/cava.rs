// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-astal
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Input};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalCavaCava")]
    pub struct Cava(Object<ffi::AstalCavaCava, ffi::AstalCavaCavaClass>);

    match fn {
        type_ => || ffi::astal_cava_cava_get_type(),
    }
}

impl Cava {
    #[doc(alias = "astal_cava_cava_get_active")]
    #[doc(alias = "get_active")]
    #[doc(alias = "active")]
    pub fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_cava_cava_get_active(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_cava_cava_get_autosens")]
    #[doc(alias = "get_autosens")]
    #[doc(alias = "autosens")]
    pub fn is_autosens(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_cava_cava_get_autosens(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_cava_cava_get_bars")]
    #[doc(alias = "get_bars")]
    pub fn bars(&self) -> i32 {
        unsafe {
            ffi::astal_cava_cava_get_bars(self.to_glib_none().0)
        }
    }

    #[doc(alias = "astal_cava_cava_get_channels")]
    #[doc(alias = "get_channels")]
    pub fn channels(&self) -> i32 {
        unsafe {
            ffi::astal_cava_cava_get_channels(self.to_glib_none().0)
        }
    }

    #[doc(alias = "astal_cava_cava_get_framerate")]
    #[doc(alias = "get_framerate")]
    pub fn framerate(&self) -> i32 {
        unsafe {
            ffi::astal_cava_cava_get_framerate(self.to_glib_none().0)
        }
    }

    #[doc(alias = "astal_cava_cava_get_high_cutoff")]
    #[doc(alias = "get_high_cutoff")]
    #[doc(alias = "high-cutoff")]
    pub fn high_cutoff(&self) -> i32 {
        unsafe {
            ffi::astal_cava_cava_get_high_cutoff(self.to_glib_none().0)
        }
    }

    #[doc(alias = "astal_cava_cava_get_input")]
    #[doc(alias = "get_input")]
    pub fn input(&self) -> Input {
        unsafe {
            from_glib(ffi::astal_cava_cava_get_input(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_cava_cava_get_low_cutoff")]
    #[doc(alias = "get_low_cutoff")]
    #[doc(alias = "low-cutoff")]
    pub fn low_cutoff(&self) -> i32 {
        unsafe {
            ffi::astal_cava_cava_get_low_cutoff(self.to_glib_none().0)
        }
    }

    #[doc(alias = "astal_cava_cava_get_noise_reduction")]
    #[doc(alias = "get_noise_reduction")]
    #[doc(alias = "noise-reduction")]
    pub fn noise_reduction(&self) -> f64 {
        unsafe {
            ffi::astal_cava_cava_get_noise_reduction(self.to_glib_none().0)
        }
    }

    #[doc(alias = "astal_cava_cava_get_samplerate")]
    #[doc(alias = "get_samplerate")]
    pub fn samplerate(&self) -> i32 {
        unsafe {
            ffi::astal_cava_cava_get_samplerate(self.to_glib_none().0)
        }
    }

    #[doc(alias = "astal_cava_cava_get_source")]
    #[doc(alias = "get_source")]
    pub fn source(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_cava_cava_get_source(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_cava_cava_get_stereo")]
    #[doc(alias = "get_stereo")]
    #[doc(alias = "stereo")]
    pub fn is_stereo(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_cava_cava_get_stereo(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_cava_cava_set_active")]
    #[doc(alias = "active")]
    pub fn set_active(&self, active: bool) {
        unsafe {
            ffi::astal_cava_cava_set_active(self.to_glib_none().0, active.into_glib());
        }
    }

    #[doc(alias = "astal_cava_cava_set_autosens")]
    #[doc(alias = "autosens")]
    pub fn set_autosens(&self, autosens: bool) {
        unsafe {
            ffi::astal_cava_cava_set_autosens(self.to_glib_none().0, autosens.into_glib());
        }
    }

    #[doc(alias = "astal_cava_cava_set_bars")]
    #[doc(alias = "bars")]
    pub fn set_bars(&self, bars: i32) {
        unsafe {
            ffi::astal_cava_cava_set_bars(self.to_glib_none().0, bars);
        }
    }

    #[doc(alias = "astal_cava_cava_set_channels")]
    #[doc(alias = "channels")]
    pub fn set_channels(&self, channels: i32) {
        unsafe {
            ffi::astal_cava_cava_set_channels(self.to_glib_none().0, channels);
        }
    }

    #[doc(alias = "astal_cava_cava_set_framerate")]
    #[doc(alias = "framerate")]
    pub fn set_framerate(&self, framerate: i32) {
        unsafe {
            ffi::astal_cava_cava_set_framerate(self.to_glib_none().0, framerate);
        }
    }

    #[doc(alias = "astal_cava_cava_set_high_cutoff")]
    #[doc(alias = "high-cutoff")]
    pub fn set_high_cutoff(&self, high_cutoff: i32) {
        unsafe {
            ffi::astal_cava_cava_set_high_cutoff(self.to_glib_none().0, high_cutoff);
        }
    }

    #[doc(alias = "astal_cava_cava_set_input")]
    #[doc(alias = "input")]
    pub fn set_input(&self, input: Input) {
        unsafe {
            ffi::astal_cava_cava_set_input(self.to_glib_none().0, input.into_glib());
        }
    }

    #[doc(alias = "astal_cava_cava_set_low_cutoff")]
    #[doc(alias = "low-cutoff")]
    pub fn set_low_cutoff(&self, low_cutoff: i32) {
        unsafe {
            ffi::astal_cava_cava_set_low_cutoff(self.to_glib_none().0, low_cutoff);
        }
    }

    #[doc(alias = "astal_cava_cava_set_noise_reduction")]
    #[doc(alias = "noise-reduction")]
    pub fn set_noise_reduction(&self, noise: f64) {
        unsafe {
            ffi::astal_cava_cava_set_noise_reduction(self.to_glib_none().0, noise);
        }
    }

    #[doc(alias = "astal_cava_cava_set_samplerate")]
    #[doc(alias = "samplerate")]
    pub fn set_samplerate(&self, samplerate: i32) {
        unsafe {
            ffi::astal_cava_cava_set_samplerate(self.to_glib_none().0, samplerate);
        }
    }

    #[doc(alias = "astal_cava_cava_set_source")]
    #[doc(alias = "source")]
    pub fn set_source(&self, source: &str) {
        unsafe {
            ffi::astal_cava_cava_set_source(self.to_glib_none().0, source.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_cava_cava_set_stereo")]
    #[doc(alias = "stereo")]
    pub fn set_stereo(&self, stereo: bool) {
        unsafe {
            ffi::astal_cava_cava_set_stereo(self.to_glib_none().0, stereo.into_glib());
        }
    }

    #[doc(alias = "astal_cava_cava_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]    pub fn default() -> Option<Cava> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::astal_cava_cava_get_default())
        }
    }

    #[doc(alias = "active")]
    pub fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_active_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "autosens")]
    pub fn connect_autosens_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autosens_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::autosens\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_autosens_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "bars")]
    pub fn connect_bars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bars_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::bars\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_bars_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "channels")]
    pub fn connect_channels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_channels_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::channels\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_channels_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "framerate")]
    pub fn connect_framerate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_framerate_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::framerate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_framerate_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "high-cutoff")]
    pub fn connect_high_cutoff_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_high_cutoff_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::high-cutoff\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_high_cutoff_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "input")]
    pub fn connect_input_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::input\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_input_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "low-cutoff")]
    pub fn connect_low_cutoff_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_low_cutoff_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::low-cutoff\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_low_cutoff_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "noise-reduction")]
    pub fn connect_noise_reduction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_noise_reduction_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::noise-reduction\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_noise_reduction_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "samplerate")]
    pub fn connect_samplerate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_samplerate_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::samplerate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_samplerate_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "source")]
    pub fn connect_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_source_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::source\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_source_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "stereo")]
    pub fn connect_stereo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stereo_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::stereo\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_stereo_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "values")]
    pub fn connect_values_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_values_trampoline<F: Fn(&Cava) + 'static>(this: *mut ffi::AstalCavaCava, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::values\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_values_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
