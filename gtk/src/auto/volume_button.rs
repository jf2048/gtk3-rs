// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Actionable, Adjustment, Align, Bin, Buildable, Button, Container, IconSize, Orientable,
    Orientation, PositionType, ReliefStyle, ResizeMode, ScaleButton, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkVolumeButton")]
    pub struct VolumeButton(Object<ffi::GtkVolumeButton, ffi::GtkVolumeButtonClass>) @extends ScaleButton, Button, Bin, Container, Widget, @implements Buildable, Actionable, Orientable;

    match fn {
        type_ => || ffi::gtk_volume_button_get_type(),
    }
}

impl VolumeButton {
    pub const NONE: Option<&'static VolumeButton> = None;

    #[doc(alias = "gtk_volume_button_new")]
    pub fn new() -> VolumeButton {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_volume_button_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`VolumeButton`] objects.
    ///
    /// This method returns an instance of [`VolumeButtonBuilder`](crate::builders::VolumeButtonBuilder) which can be used to create [`VolumeButton`] objects.
    pub fn builder() -> VolumeButtonBuilder {
        VolumeButtonBuilder::new()
    }
}

impl Default for VolumeButton {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`VolumeButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct VolumeButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, VolumeButton>,
}

impl VolumeButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn use_symbolic(self, use_symbolic: bool) -> Self {
        Self {
            builder: self.builder.property("use-symbolic", use_symbolic),
        }
    }

    pub fn adjustment(self, adjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("adjustment", adjustment.clone().upcast()),
        }
    }

    pub fn icons(self, icons: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("icons", icons.into()),
        }
    }

    pub fn size(self, size: IconSize) -> Self {
        Self {
            builder: self.builder.property("size", size),
        }
    }

    pub fn value(self, value: f64) -> Self {
        Self {
            builder: self.builder.property("value", value),
        }
    }

    pub fn always_show_image(self, always_show_image: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("always-show-image", always_show_image),
        }
    }

    pub fn image(self, image: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("image", image.clone().upcast()),
        }
    }

    pub fn image_position(self, image_position: PositionType) -> Self {
        Self {
            builder: self.builder.property("image-position", image_position),
        }
    }

    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    pub fn relief(self, relief: ReliefStyle) -> Self {
        Self {
            builder: self.builder.property("relief", relief),
        }
    }

    pub fn use_underline(self, use_underline: bool) -> Self {
        Self {
            builder: self.builder.property("use-underline", use_underline),
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

    pub fn action_name(self, action_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("action-name", action_name.into()),
        }
    }

    pub fn action_target(self, action_target: &glib::Variant) -> Self {
        Self {
            builder: self
                .builder
                .property("action-target", action_target.clone()),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`VolumeButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> VolumeButton {
        self.builder.build()
    }
}

pub trait VolumeButtonExt: IsA<VolumeButton> + 'static {
    #[doc(alias = "use-symbolic")]
    fn uses_symbolic(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "use-symbolic")
    }

    #[doc(alias = "use-symbolic")]
    fn set_use_symbolic(&self, use_symbolic: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "use-symbolic", use_symbolic)
    }

    #[doc(alias = "use-symbolic")]
    fn connect_use_symbolic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_symbolic_trampoline<
            P: IsA<VolumeButton>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkVolumeButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VolumeButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-symbolic\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_symbolic_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<VolumeButton>> VolumeButtonExt for O {}

impl fmt::Display for VolumeButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("VolumeButton")
    }
}
