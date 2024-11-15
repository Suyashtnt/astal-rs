// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-astal
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Exclusivity,Keymode,Layer,WindowAnchor};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalWindow")]
    pub struct Window(Object<ffi::AstalWindow, ffi::AstalWindowClass>) @extends gtk::Window, gtk::Widget, gobject::InitiallyUnowned, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;

    match fn {
        type_ => || ffi::astal_window_get_type(),
    }
}

impl Window {
        pub const NONE: Option<&'static Window> = None;
    

    #[doc(alias = "astal_window_new")]
    pub fn new() -> Window {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::astal_window_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Window`] objects.
            ///
            /// This method returns an instance of [`WindowBuilder`](crate::builders::WindowBuilder) which can be used to create [`Window`] objects.
            pub fn builder() -> WindowBuilder {
                WindowBuilder::new()
            }
        
}

impl Default for Window {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Window`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct WindowBuilder {
            builder: glib::object::ObjectBuilder<'static, Window>,
        }

        impl WindowBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn namespace(self, namespace: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("namespace", namespace.into()), }
                        }

                            pub fn anchor(self, anchor: WindowAnchor) -> Self {
                            Self { builder: self.builder.property("anchor", anchor), }
                        }

                            pub fn exclusivity(self, exclusivity: Exclusivity) -> Self {
                            Self { builder: self.builder.property("exclusivity", exclusivity), }
                        }

                            pub fn layer(self, layer: Layer) -> Self {
                            Self { builder: self.builder.property("layer", layer), }
                        }

                            pub fn keymode(self, keymode: Keymode) -> Self {
                            Self { builder: self.builder.property("keymode", keymode), }
                        }

                            pub fn gdkmonitor(self, gdkmonitor: &gdk::Monitor) -> Self {
                            Self { builder: self.builder.property("gdkmonitor", gdkmonitor.clone()), }
                        }

                            pub fn margin_top(self, margin_top: i32) -> Self {
                            Self { builder: self.builder.property("margin-top", margin_top), }
                        }

                            pub fn margin_bottom(self, margin_bottom: i32) -> Self {
                            Self { builder: self.builder.property("margin-bottom", margin_bottom), }
                        }

                            pub fn margin_left(self, margin_left: i32) -> Self {
                            Self { builder: self.builder.property("margin-left", margin_left), }
                        }

                            pub fn margin_right(self, margin_right: i32) -> Self {
                            Self { builder: self.builder.property("margin-right", margin_right), }
                        }

                            pub fn margin(self, margin: i32) -> Self {
                            Self { builder: self.builder.property("margin", margin), }
                        }

                            pub fn monitor(self, monitor: i32) -> Self {
                            Self { builder: self.builder.property("monitor", monitor), }
                        }

                            pub fn application(self, application: &impl IsA<gtk::Application>) -> Self {
                            Self { builder: self.builder.property("application", application.clone().upcast()), }
                        }

                            pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
                            Self { builder: self.builder.property("child", child.clone().upcast()), }
                        }

                            pub fn decorated(self, decorated: bool) -> Self {
                            Self { builder: self.builder.property("decorated", decorated), }
                        }

                            pub fn default_height(self, default_height: i32) -> Self {
                            Self { builder: self.builder.property("default-height", default_height), }
                        }

                            pub fn default_widget(self, default_widget: &impl IsA<gtk::Widget>) -> Self {
                            Self { builder: self.builder.property("default-widget", default_widget.clone().upcast()), }
                        }

                            pub fn default_width(self, default_width: i32) -> Self {
                            Self { builder: self.builder.property("default-width", default_width), }
                        }

                            pub fn deletable(self, deletable: bool) -> Self {
                            Self { builder: self.builder.property("deletable", deletable), }
                        }

                            pub fn destroy_with_parent(self, destroy_with_parent: bool) -> Self {
                            Self { builder: self.builder.property("destroy-with-parent", destroy_with_parent), }
                        }

                            //pub fn display(self, display: /*Ignored*/&gdk::Display) -> Self {
                        //    Self { builder: self.builder.property("display", display), }
                        //}

                            pub fn focus_visible(self, focus_visible: bool) -> Self {
                            Self { builder: self.builder.property("focus-visible", focus_visible), }
                        }

                            pub fn focus_widget(self, focus_widget: &impl IsA<gtk::Widget>) -> Self {
                            Self { builder: self.builder.property("focus-widget", focus_widget.clone().upcast()), }
                        }

                            pub fn fullscreened(self, fullscreened: bool) -> Self {
                            Self { builder: self.builder.property("fullscreened", fullscreened), }
                        }

                            #[cfg(feature = "gtk_v4_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v4_2")))]
    pub fn handle_menubar_accel(self, handle_menubar_accel: bool) -> Self {
                            Self { builder: self.builder.property("handle-menubar-accel", handle_menubar_accel), }
                        }

                            pub fn hide_on_close(self, hide_on_close: bool) -> Self {
                            Self { builder: self.builder.property("hide-on-close", hide_on_close), }
                        }

                            pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("icon-name", icon_name.into()), }
                        }

                            pub fn maximized(self, maximized: bool) -> Self {
                            Self { builder: self.builder.property("maximized", maximized), }
                        }

                            pub fn mnemonics_visible(self, mnemonics_visible: bool) -> Self {
                            Self { builder: self.builder.property("mnemonics-visible", mnemonics_visible), }
                        }

                            pub fn modal(self, modal: bool) -> Self {
                            Self { builder: self.builder.property("modal", modal), }
                        }

                            pub fn resizable(self, resizable: bool) -> Self {
                            Self { builder: self.builder.property("resizable", resizable), }
                        }

                            pub fn startup_id(self, startup_id: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("startup-id", startup_id.into()), }
                        }

                            pub fn title(self, title: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("title", title.into()), }
                        }

                            #[cfg(feature = "gtk_v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v4_6")))]
    pub fn titlebar(self, titlebar: &impl IsA<gtk::Widget>) -> Self {
                            Self { builder: self.builder.property("titlebar", titlebar.clone().upcast()), }
                        }

                            pub fn transient_for(self, transient_for: &impl IsA<gtk::Window>) -> Self {
                            Self { builder: self.builder.property("transient-for", transient_for.clone().upcast()), }
                        }

                            pub fn can_focus(self, can_focus: bool) -> Self {
                            Self { builder: self.builder.property("can-focus", can_focus), }
                        }

                            pub fn can_target(self, can_target: bool) -> Self {
                            Self { builder: self.builder.property("can-target", can_target), }
                        }

                            pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
                            Self { builder: self.builder.property("css-classes", css_classes.into()), }
                        }

                            pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("css-name", css_name.into()), }
                        }

                            //pub fn cursor(self, cursor: /*Ignored*/&gdk::Cursor) -> Self {
                        //    Self { builder: self.builder.property("cursor", cursor), }
                        //}

                            pub fn focus_on_click(self, focus_on_click: bool) -> Self {
                            Self { builder: self.builder.property("focus-on-click", focus_on_click), }
                        }

                            pub fn focusable(self, focusable: bool) -> Self {
                            Self { builder: self.builder.property("focusable", focusable), }
                        }

                            //pub fn halign(self, halign: /*Ignored*/gtk::Align) -> Self {
                        //    Self { builder: self.builder.property("halign", halign), }
                        //}

                            pub fn has_tooltip(self, has_tooltip: bool) -> Self {
                            Self { builder: self.builder.property("has-tooltip", has_tooltip), }
                        }

                            pub fn height_request(self, height_request: i32) -> Self {
                            Self { builder: self.builder.property("height-request", height_request), }
                        }

                            pub fn hexpand(self, hexpand: bool) -> Self {
                            Self { builder: self.builder.property("hexpand", hexpand), }
                        }

                            pub fn hexpand_set(self, hexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("hexpand-set", hexpand_set), }
                        }

                            //pub fn layout_manager(self, layout_manager: &impl IsA</*Ignored*/gtk::LayoutManager>) -> Self {
                        //    Self { builder: self.builder.property("layout-manager", layout_manager.clone().upcast()), }
                        //}

                            pub fn margin_end(self, margin_end: i32) -> Self {
                            Self { builder: self.builder.property("margin-end", margin_end), }
                        }

                            pub fn margin_start(self, margin_start: i32) -> Self {
                            Self { builder: self.builder.property("margin-start", margin_start), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn opacity(self, opacity: f64) -> Self {
                            Self { builder: self.builder.property("opacity", opacity), }
                        }

                            //pub fn overflow(self, overflow: /*Ignored*/gtk::Overflow) -> Self {
                        //    Self { builder: self.builder.property("overflow", overflow), }
                        //}

                            pub fn receives_default(self, receives_default: bool) -> Self {
                            Self { builder: self.builder.property("receives-default", receives_default), }
                        }

                            pub fn sensitive(self, sensitive: bool) -> Self {
                            Self { builder: self.builder.property("sensitive", sensitive), }
                        }

                            pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-markup", tooltip_markup.into()), }
                        }

                            pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-text", tooltip_text.into()), }
                        }

                            //pub fn valign(self, valign: /*Ignored*/gtk::Align) -> Self {
                        //    Self { builder: self.builder.property("valign", valign), }
                        //}

                            pub fn vexpand(self, vexpand: bool) -> Self {
                            Self { builder: self.builder.property("vexpand", vexpand), }
                        }

                            pub fn vexpand_set(self, vexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("vexpand-set", vexpand_set), }
                        }

                            pub fn visible(self, visible: bool) -> Self {
                            Self { builder: self.builder.property("visible", visible), }
                        }

                            pub fn width_request(self, width_request: i32) -> Self {
                            Self { builder: self.builder.property("width-request", width_request), }
                        }

                            //pub fn accessible_role(self, accessible_role: /*Ignored*/gtk::AccessibleRole) -> Self {
                        //    Self { builder: self.builder.property("accessible-role", accessible_role), }
                        //}

    // rustdoc-stripper-ignore-next
    /// Build the [`Window`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Window {
assert_initialized_main_thread!();
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Window>> Sealed for T {}
}

pub trait WindowExt: IsA<Window> + sealed::Sealed + 'static {
    #[doc(alias = "astal_window_get_namespace")]
    #[doc(alias = "get_namespace")]
    fn namespace(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::astal_window_get_namespace(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_window_set_namespace")]
    fn set_namespace(&self, value: &str) {
        unsafe {
            ffi::astal_window_set_namespace(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_window_get_anchor")]
    #[doc(alias = "get_anchor")]
    fn anchor(&self) -> WindowAnchor {
        unsafe {
            from_glib(ffi::astal_window_get_anchor(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_window_set_anchor")]
    fn set_anchor(&self, value: WindowAnchor) {
        unsafe {
            ffi::astal_window_set_anchor(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_window_get_exclusivity")]
    #[doc(alias = "get_exclusivity")]
    fn exclusivity(&self) -> Exclusivity {
        unsafe {
            from_glib(ffi::astal_window_get_exclusivity(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_window_set_exclusivity")]
    fn set_exclusivity(&self, value: Exclusivity) {
        unsafe {
            ffi::astal_window_set_exclusivity(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_window_get_layer")]
    #[doc(alias = "get_layer")]
    fn layer(&self) -> Layer {
        unsafe {
            from_glib(ffi::astal_window_get_layer(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_window_set_layer")]
    fn set_layer(&self, value: Layer) {
        unsafe {
            ffi::astal_window_set_layer(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_window_get_keymode")]
    #[doc(alias = "get_keymode")]
    fn keymode(&self) -> Keymode {
        unsafe {
            from_glib(ffi::astal_window_get_keymode(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_window_set_keymode")]
    fn set_keymode(&self, value: Keymode) {
        unsafe {
            ffi::astal_window_set_keymode(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_window_get_gdkmonitor")]
    #[doc(alias = "get_gdkmonitor")]
    fn gdkmonitor(&self) -> Option<gdk::Monitor> {
        unsafe {
            from_glib_none(ffi::astal_window_get_gdkmonitor(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_window_set_gdkmonitor")]
    fn set_gdkmonitor(&self, value: &gdk::Monitor) {
        unsafe {
            ffi::astal_window_set_gdkmonitor(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_window_get_margin_top")]
    #[doc(alias = "get_margin_top")]
    fn margin_top(&self) -> i32 {
        unsafe {
            ffi::astal_window_get_margin_top(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_window_set_margin_top")]
    fn set_margin_top(&self, value: i32) {
        unsafe {
            ffi::astal_window_set_margin_top(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_window_get_margin_bottom")]
    #[doc(alias = "get_margin_bottom")]
    fn margin_bottom(&self) -> i32 {
        unsafe {
            ffi::astal_window_get_margin_bottom(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_window_set_margin_bottom")]
    fn set_margin_bottom(&self, value: i32) {
        unsafe {
            ffi::astal_window_set_margin_bottom(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_window_get_margin_left")]
    #[doc(alias = "get_margin_left")]
    fn margin_left(&self) -> i32 {
        unsafe {
            ffi::astal_window_get_margin_left(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_window_set_margin_left")]
    fn set_margin_left(&self, value: i32) {
        unsafe {
            ffi::astal_window_set_margin_left(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_window_get_margin_right")]
    #[doc(alias = "get_margin_right")]
    fn margin_right(&self) -> i32 {
        unsafe {
            ffi::astal_window_get_margin_right(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_window_set_margin_right")]
    fn set_margin_right(&self, value: i32) {
        unsafe {
            ffi::astal_window_set_margin_right(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_window_set_margin")]
    fn set_margin(&self, value: i32) {
        unsafe {
            ffi::astal_window_set_margin(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_window_get_monitor")]
    #[doc(alias = "get_monitor")]
    fn monitor(&self) -> i32 {
        unsafe {
            ffi::astal_window_get_monitor(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_window_set_monitor")]
    fn set_monitor(&self, value: i32) {
        unsafe {
            ffi::astal_window_set_monitor(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "namespace")]
    fn connect_namespace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_namespace_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(this: *mut ffi::AstalWindow, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::namespace\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_namespace_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "anchor")]
    fn connect_anchor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_anchor_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(this: *mut ffi::AstalWindow, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::anchor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_anchor_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "exclusivity")]
    fn connect_exclusivity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_exclusivity_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(this: *mut ffi::AstalWindow, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::exclusivity\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_exclusivity_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "layer")]
    fn connect_layer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_layer_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(this: *mut ffi::AstalWindow, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::layer\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_layer_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "keymode")]
    fn connect_keymode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_keymode_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(this: *mut ffi::AstalWindow, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::keymode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_keymode_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "gdkmonitor")]
    fn connect_gdkmonitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gdkmonitor_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(this: *mut ffi::AstalWindow, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gdkmonitor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_gdkmonitor_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "margin-left")]
    fn connect_margin_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_margin_left_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(this: *mut ffi::AstalWindow, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::margin-left\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_margin_left_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "margin-right")]
    fn connect_margin_right_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_margin_right_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(this: *mut ffi::AstalWindow, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::margin-right\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_margin_right_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "margin")]
    fn connect_margin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_margin_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(this: *mut ffi::AstalWindow, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::margin\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_margin_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "monitor")]
    fn connect_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_monitor_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(this: *mut ffi::AstalWindow, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::monitor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_monitor_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Window>> WindowExt for O {}