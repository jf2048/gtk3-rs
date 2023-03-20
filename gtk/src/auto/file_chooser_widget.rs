// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, BaselinePosition, Box, Buildable, Container, FileChooser, FileChooserAction, FileFilter,
    Orientable, Orientation, ResizeMode, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkFileChooserWidget")]
    pub struct FileChooserWidget(Object<ffi::GtkFileChooserWidget, ffi::GtkFileChooserWidgetClass>) @extends Box, Container, Widget, @implements Buildable, Orientable, FileChooser;

    match fn {
        type_ => || ffi::gtk_file_chooser_widget_get_type(),
    }
}

impl FileChooserWidget {
    pub const NONE: Option<&'static FileChooserWidget> = None;

    #[doc(alias = "gtk_file_chooser_widget_new")]
    pub fn new(action: FileChooserAction) -> FileChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_widget_new(action.into_glib()))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FileChooserWidget`] objects.
    ///
    /// This method returns an instance of [`FileChooserWidgetBuilder`](crate::builders::FileChooserWidgetBuilder) which can be used to create [`FileChooserWidget`] objects.
    pub fn builder() -> FileChooserWidgetBuilder {
        FileChooserWidgetBuilder::new()
    }
}

impl Default for FileChooserWidget {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FileChooserWidget`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FileChooserWidgetBuilder {
    builder: glib::object::ObjectBuilder<'static, FileChooserWidget>,
}

impl FileChooserWidgetBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn search_mode(self, search_mode: bool) -> Self {
        Self {
            builder: self.builder.property("search-mode", search_mode),
        }
    }

    pub fn baseline_position(self, baseline_position: BaselinePosition) -> Self {
        Self {
            builder: self
                .builder
                .property("baseline-position", baseline_position),
        }
    }

    pub fn homogeneous(self, homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("homogeneous", homogeneous),
        }
    }

    pub fn spacing(self, spacing: i32) -> Self {
        Self {
            builder: self.builder.property("spacing", spacing),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    pub fn action(self, action: FileChooserAction) -> Self {
        Self {
            builder: self.builder.property("action", action),
        }
    }

    pub fn create_folders(self, create_folders: bool) -> Self {
        Self {
            builder: self.builder.property("create-folders", create_folders),
        }
    }

    pub fn do_overwrite_confirmation(self, do_overwrite_confirmation: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("do-overwrite-confirmation", do_overwrite_confirmation),
        }
    }

    pub fn extra_widget(self, extra_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("extra-widget", extra_widget.clone().upcast()),
        }
    }

    pub fn filter(self, filter: &FileFilter) -> Self {
        Self {
            builder: self.builder.property("filter", filter.clone()),
        }
    }

    pub fn local_only(self, local_only: bool) -> Self {
        Self {
            builder: self.builder.property("local-only", local_only),
        }
    }

    pub fn preview_widget(self, preview_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("preview-widget", preview_widget.clone().upcast()),
        }
    }

    pub fn preview_widget_active(self, preview_widget_active: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("preview-widget-active", preview_widget_active),
        }
    }

    pub fn select_multiple(self, select_multiple: bool) -> Self {
        Self {
            builder: self.builder.property("select-multiple", select_multiple),
        }
    }

    pub fn show_hidden(self, show_hidden: bool) -> Self {
        Self {
            builder: self.builder.property("show-hidden", show_hidden),
        }
    }

    pub fn use_preview_label(self, use_preview_label: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("use-preview-label", use_preview_label),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FileChooserWidget`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FileChooserWidget {
        self.builder.build()
    }
}

pub trait FileChooserWidgetExt: 'static {
    #[doc(alias = "search-mode")]
    fn is_search_mode(&self) -> bool;

    #[doc(alias = "search-mode")]
    fn set_search_mode(&self, search_mode: bool);

    fn subtitle(&self) -> Option<glib::GString>;

    #[doc(alias = "desktop-folder")]
    fn connect_desktop_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_desktop_folder(&self);

    #[doc(alias = "down-folder")]
    fn connect_down_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_down_folder(&self);

    #[doc(alias = "home-folder")]
    fn connect_home_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_home_folder(&self);

    #[doc(alias = "location-popup")]
    fn connect_location_popup<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_location_popup(&self, path: &str);

    #[doc(alias = "location-popup-on-paste")]
    fn connect_location_popup_on_paste<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_location_popup_on_paste(&self);

    #[doc(alias = "location-toggle-popup")]
    fn connect_location_toggle_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_location_toggle_popup(&self);

    #[doc(alias = "places-shortcut")]
    fn connect_places_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_places_shortcut(&self);

    #[doc(alias = "quick-bookmark")]
    fn connect_quick_bookmark<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_quick_bookmark(&self, bookmark_index: i32);

    #[doc(alias = "recent-shortcut")]
    fn connect_recent_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_recent_shortcut(&self);

    #[doc(alias = "search-shortcut")]
    fn connect_search_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_search_shortcut(&self);

    #[doc(alias = "show-hidden")]
    fn connect_show_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_show_hidden(&self);

    #[doc(alias = "up-folder")]
    fn connect_up_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_up_folder(&self);

    #[doc(alias = "search-mode")]
    fn connect_search_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "subtitle")]
    fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooserWidget>> FileChooserWidgetExt for O {
    fn is_search_mode(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "search-mode")
    }

    fn set_search_mode(&self, search_mode: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "search-mode", search_mode)
    }

    fn subtitle(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "subtitle")
    }

    fn connect_desktop_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn desktop_folder_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"desktop-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    desktop_folder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_desktop_folder(&self) {
        self.emit_by_name::<()>("desktop-folder", &[]);
    }

    fn connect_down_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn down_folder_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"down-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    down_folder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_down_folder(&self) {
        self.emit_by_name::<()>("down-folder", &[]);
    }

    fn connect_home_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn home_folder_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"home-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    home_folder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_home_folder(&self) {
        self.emit_by_name::<()>("home-folder", &[]);
    }

    fn connect_location_popup<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn location_popup_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P, &str) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            path: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(path),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    location_popup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_location_popup(&self, path: &str) {
        self.emit_by_name::<()>("location-popup", &[&path]);
    }

    fn connect_location_popup_on_paste<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn location_popup_on_paste_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-popup-on-paste\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    location_popup_on_paste_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_location_popup_on_paste(&self) {
        self.emit_by_name::<()>("location-popup-on-paste", &[]);
    }

    fn connect_location_toggle_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn location_toggle_popup_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-toggle-popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    location_toggle_popup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_location_toggle_popup(&self) {
        self.emit_by_name::<()>("location-toggle-popup", &[]);
    }

    fn connect_places_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn places_shortcut_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"places-shortcut\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    places_shortcut_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_places_shortcut(&self) {
        self.emit_by_name::<()>("places-shortcut", &[]);
    }

    fn connect_quick_bookmark<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn quick_bookmark_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P, i32) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            bookmark_index: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref(),
                bookmark_index,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"quick-bookmark\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    quick_bookmark_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_quick_bookmark(&self, bookmark_index: i32) {
        self.emit_by_name::<()>("quick-bookmark", &[&bookmark_index]);
    }

    fn connect_recent_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn recent_shortcut_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"recent-shortcut\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    recent_shortcut_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_recent_shortcut(&self) {
        self.emit_by_name::<()>("recent-shortcut", &[]);
    }

    fn connect_search_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn search_shortcut_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"search-shortcut\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    search_shortcut_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_search_shortcut(&self) {
        self.emit_by_name::<()>("search-shortcut", &[]);
    }

    fn connect_show_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_hidden_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-hidden\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_hidden_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_show_hidden(&self) {
        self.emit_by_name::<()>("show-hidden", &[]);
    }

    fn connect_up_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn up_folder_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"up-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    up_folder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_up_folder(&self) {
        self.emit_by_name::<()>("up-folder", &[]);
    }

    fn connect_search_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_mode_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<
            P: IsA<FileChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FileChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileChooserWidget")
    }
}
