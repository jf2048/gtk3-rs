use gtk_sys;

use libc::c_int;
use std::mem;

use glib::translate::*;

use glib::subclass::prelude::*;
use glib::{Cast, Object};

use crate::DragResult;
use crate::Inhibit;
use crate::Orientation;
use crate::SelectionData;
use crate::TextDirection;
use cairo;
use cairo_sys;
use Allocation;
use SizeRequestMode;
use Widget;
use WidgetExt;

pub trait WidgetImpl: WidgetImplExt + ObjectImpl {
    fn adjust_baseline_allocation(&self, widget: &Self::Type, baseline: &mut i32) {
        self.parent_adjust_baseline_allocation(widget, baseline)
    }

    fn adjust_baseline_request(
        &self,
        widget: &Self::Type,
        minimum_baseline: &mut i32,
        natural_baseline: &mut i32,
    ) {
        self.parent_adjust_baseline_request(widget, minimum_baseline, natural_baseline)
    }

    fn adjust_size_allocation(
        &self,
        widget: &Self::Type,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    ) {
        self.parent_adjust_size_allocation(
            widget,
            orientation,
            minimum_size,
            natural_size,
            allocated_pos,
            allocated_size,
        )
    }

    fn adjust_size_request(
        &self,
        widget: &Self::Type,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    ) {
        self.parent_adjust_size_request(widget, orientation, minimum_size, natural_size)
    }

    fn button_press_event(&self, widget: &Self::Type, event: &gdk::EventButton) -> Inhibit {
        self.parent_button_press_event(widget, event)
    }

    fn button_release_event(&self, widget: &Self::Type, event: &gdk::EventButton) -> Inhibit {
        self.parent_button_release_event(widget, event)
    }

    fn child_notify(&self, widget: &Self::Type, child_property: &glib::ParamSpec) {
        self.parent_child_notify(widget, child_property)
    }

    fn composited_changed(&self, widget: &Self::Type) {
        self.parent_composited_changed(widget)
    }

    fn compute_expand(&self, widget: &Self::Type, hexpand_p: &mut bool, vexpand_p: &mut bool) {
        self.parent_compute_expand(widget, hexpand_p, vexpand_p)
    }

    fn configure_event(&self, widget: &Self::Type, event: &gdk::EventConfigure) -> Inhibit {
        self.parent_configure_event(widget, event)
    }

    fn damage_event(&self, widget: &Self::Type, event: &gdk::EventExpose) -> Inhibit {
        self.parent_damage_event(widget, event)
    }

    fn delete_event(&self, widget: &Self::Type, event: &gdk::Event) -> Inhibit {
        self.parent_delete_event(widget, event)
    }

    fn destroy(&self, widget: &Self::Type) {
        self.parent_destroy(widget)
    }

    fn destroy_event(&self, widget: &Self::Type, event: &gdk::Event) -> Inhibit {
        self.parent_destroy_event(widget, event)
    }

    fn direction_changed(&self, widget: &Self::Type, previous_direction: TextDirection) {
        self.parent_direction_changed(widget, previous_direction)
    }

    fn dispatch_child_properties_changed(&self, widget: &Self::Type, pspecs: &[glib::ParamSpec]) {
        self.parent_dispatch_child_properties_changed(widget, pspecs)
    }

    fn drag_begin(&self, widget: &Self::Type, context: &gdk::DragContext) {
        self.parent_drag_begin(widget, context)
    }

    fn drag_data_delete(&self, widget: &Self::Type, context: &gdk::DragContext) {
        self.parent_drag_data_delete(widget, context)
    }

    fn drag_data_get(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        self.parent_drag_data_get(widget, context, selection_data, info, time)
    }

    fn drag_data_received(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        self.parent_drag_data_received(widget, context, x, y, selection_data, info, time)
    }

    fn drag_drop(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit {
        self.parent_drag_drop(widget, context, x, y, time)
    }

    fn drag_end(&self, widget: &Self::Type, context: &gdk::DragContext) {
        self.parent_drag_end(widget, context)
    }

    fn drag_failed(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        result: DragResult,
    ) -> Inhibit {
        self.parent_drag_failed(widget, context, result)
    }

    fn drag_leave(&self, widget: &Self::Type, context: &gdk::DragContext, time: u32) {
        self.parent_drag_leave(widget, context, time)
    }

    fn drag_motion(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit {
        self.parent_drag_motion(widget, context, x, y, time)
    }

    fn draw(&self, widget: &Self::Type, cr: &cairo::Context) -> Inhibit {
        self.parent_draw(widget, cr)
    }

    // fn can_activate_accel(&self, widget: &Self::Type, signal_id: u32) -> bool {
    //     self.parent_can_activate_accel(widget, signal_id)
    // }

    fn get_request_mode(&self, widget: &Self::Type) -> SizeRequestMode {
        self.parent_get_request_mode(widget)
    }

    fn get_preferred_width(&self, widget: &Self::Type) -> (i32, i32) {
        self.parent_get_preferred_width(widget)
    }

    fn get_preferred_width_for_height(&self, widget: &Self::Type, height: i32) -> (i32, i32) {
        self.parent_get_preferred_width_for_height(widget, height)
    }

    fn get_preferred_height(&self, widget: &Self::Type) -> (i32, i32) {
        self.parent_get_preferred_height(widget)
    }

    fn get_preferred_height_for_width(&self, widget: &Self::Type, width: i32) -> (i32, i32) {
        self.parent_get_preferred_height_for_width(widget, width)
    }

    fn size_allocate(&self, widget: &Self::Type, allocation: &Allocation) {
        self.parent_size_allocate(widget, allocation)
    }

    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);
    }

    fn unrealize(&self, widget: &Self::Type) {
        self.parent_unrealize(widget);
    }
    fn map(&self, widget: &Self::Type) {
        self.parent_map(widget);
    }

    fn unmap(&self, widget: &Self::Type) {
        self.parent_unmap(widget);
    }

    fn motion_notify_event(&self, widget: &Self::Type, event: &gdk::EventMotion) -> Inhibit {
        self.parent_motion_notify_event(widget, event)
    }

    fn scroll_event(&self, widget: &Self::Type, event: &gdk::EventScroll) -> Inhibit {
        self.parent_scroll_event(widget, event)
    }
}

pub trait WidgetImplExt: ObjectSubclass {
    fn parent_adjust_baseline_allocation(&self, widget: &Self::Type, baseline: &mut i32);
    fn parent_adjust_baseline_request(
        &self,
        widget: &Self::Type,
        minimum_baseline: &mut i32,
        natural_baseline: &mut i32,
    );
    fn parent_adjust_size_allocation(
        &self,
        widget: &Self::Type,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    );
    fn parent_adjust_size_request(
        &self,
        widget: &Self::Type,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    );
    fn parent_button_press_event(&self, widget: &Self::Type, event: &gdk::EventButton) -> Inhibit;
    fn parent_button_release_event(&self, widget: &Self::Type, event: &gdk::EventButton)
        -> Inhibit;
    // fn parent_can_activate_accel(&self, widget: &Self::Type, signal_id: u32) -> bool;
    fn parent_child_notify(&self, widget: &Self::Type, child_property: &glib::ParamSpec);
    fn parent_composited_changed(&self, widget: &Self::Type);
    fn parent_compute_expand(
        &self,
        widget: &Self::Type,
        hexpand_p: &mut bool,
        vexpand_p: &mut bool,
    );
    fn parent_configure_event(&self, widget: &Self::Type, event: &gdk::EventConfigure) -> Inhibit;
    fn parent_damage_event(&self, widget: &Self::Type, event: &gdk::EventExpose) -> Inhibit;
    fn parent_delete_event(&self, widget: &Self::Type, event: &gdk::Event) -> Inhibit;
    fn parent_destroy(&self, widget: &Self::Type);
    fn parent_destroy_event(&self, widget: &Self::Type, event: &gdk::Event) -> Inhibit;
    fn parent_direction_changed(&self, widget: &Self::Type, previous_direction: TextDirection);
    fn parent_dispatch_child_properties_changed(
        &self,
        widget: &Self::Type,
        pspecs: &[glib::ParamSpec],
    );
    fn parent_drag_begin(&self, widget: &Self::Type, context: &gdk::DragContext);
    fn parent_drag_data_delete(&self, widget: &Self::Type, context: &gdk::DragContext);
    fn parent_drag_data_get(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    );
    fn parent_drag_data_received(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    );
    fn parent_drag_drop(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit;
    fn parent_drag_end(&self, widget: &Self::Type, context: &gdk::DragContext);
    fn parent_drag_failed(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        result: DragResult,
    ) -> Inhibit;
    fn parent_drag_leave(&self, widget: &Self::Type, context: &gdk::DragContext, time: u32);
    fn parent_drag_motion(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit;
    fn parent_draw(&self, widget: &Self::Type, cr: &cairo::Context) -> Inhibit;
    fn parent_get_request_mode(&self, widget: &Self::Type) -> SizeRequestMode;
    fn parent_get_preferred_width(&self, widget: &Self::Type) -> (i32, i32);
    fn parent_get_preferred_width_for_height(&self, widget: &Self::Type, height: i32)
        -> (i32, i32);
    fn parent_get_preferred_height(&self, widget: &Self::Type) -> (i32, i32);
    fn parent_get_preferred_height_for_width(&self, widget: &Self::Type, width: i32) -> (i32, i32);
    fn parent_size_allocate(&self, widget: &Self::Type, allocation: &Allocation);
    fn parent_realize(&self, widget: &Self::Type);
    fn parent_unrealize(&self, widget: &Self::Type);
    fn parent_map(&self, widget: &Self::Type);
    fn parent_unmap(&self, widget: &Self::Type);
    fn parent_motion_notify_event(&self, widget: &Self::Type, event: &gdk::EventMotion) -> Inhibit;
    fn parent_scroll_event(&self, widget: &Self::Type, event: &gdk::EventScroll) -> Inhibit;
}

impl<T: WidgetImpl> WidgetImplExt for T {
    fn parent_adjust_baseline_allocation(&self, widget: &Self::Type, baseline: &mut i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_baseline_allocation
                .expect("No parent class impl for \"adjust_baseline_allocation\"");
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                baseline,
            )
        }
    }

    fn parent_adjust_baseline_request(
        &self,
        widget: &Self::Type,
        minimum_baseline: &mut i32,
        natural_baseline: &mut i32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_baseline_request
                .expect("No parent class impl for \"adjust_baseline_request\"");
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                minimum_baseline,
                natural_baseline,
            )
        }
    }

    fn parent_adjust_size_allocation(
        &self,
        widget: &Self::Type,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_size_allocation
                .expect("No parent class impl for \"adjust_size_allocation\"");
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                orientation.to_glib(),
                minimum_size,
                natural_size,
                allocated_pos,
                allocated_size,
            )
        }
    }

    fn parent_adjust_size_request(
        &self,
        widget: &Self::Type,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_size_request
                .expect("No parent class impl for \"adjust_size_request\"");
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                orientation.to_glib(),
                minimum_size as *mut i32,
                natural_size as *mut i32,
            )
        }
    }

    fn parent_button_press_event(&self, widget: &Self::Type, event: &gdk::EventButton) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).button_press_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_button_release_event(
        &self,
        widget: &Self::Type,
        event: &gdk::EventButton,
    ) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).button_release_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    // fn parent_can_activate_accel(&self, widget: &Self::Type, signal_id: u32) -> bool {
    //     unsafe {
    //         let data = T::type_data();
    //         let parent_class =
    //             data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
    //         let f = (*parent_class)
    //             .can_activate_accel
    //             .expect("No parent class impl for \"can_activate_accel\"");
    //         f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0, signal_id) != 0
    //     }
    // }

    fn parent_child_notify(&self, widget: &Self::Type, child_property: &glib::ParamSpec) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).child_notify {
                let pspec_glib = glib::translate::mut_override(child_property.to_glib_none().0);
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    pspec_glib,
                )
            }
        }
    }

    fn parent_composited_changed(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).composited_changed {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_compute_expand(
        &self,
        widget: &Self::Type,
        hexpand_p: &mut bool,
        vexpand_p: &mut bool,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let widget = widget.unsafe_cast_ref::<Widget>();
            if let Some(f) = (*parent_class).compute_expand {
                let mut h: i32 = hexpand_p.to_glib();
                let mut v: i32 = vexpand_p.to_glib();
                f(widget.to_glib_none().0, &mut h, &mut v);
                *hexpand_p = from_glib(h);
                *vexpand_p = from_glib(v);
            } else {
                // Fill the booleans so the compiler will be happy
                // and do nothing else since the vmenthod is NULL
                *hexpand_p = widget.get_hexpand();
                *vexpand_p = widget.get_vexpand();
            }
        }
    }

    fn parent_configure_event(&self, widget: &Self::Type, event: &gdk::EventConfigure) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).configure_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_damage_event(&self, widget: &Self::Type, event: &gdk::EventExpose) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).damage_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_delete_event(&self, widget: &Self::Type, event: &gdk::Event) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).delete_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_destroy(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).destroy {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0)
            }
        }
    }

    fn parent_destroy_event(&self, widget: &Self::Type, event: &gdk::Event) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).destroy_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    ev_glib,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_direction_changed(&self, widget: &Self::Type, previous_direction: TextDirection) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).direction_changed {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    previous_direction.to_glib(),
                )
            }
        }
    }

    fn parent_dispatch_child_properties_changed(
        &self,
        widget: &Self::Type,
        pspecs: &[glib::ParamSpec],
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).dispatch_child_properties_changed {
                let mut pspecs_array = pspecs
                    .iter()
                    .map(|p| p.to_glib_none().0)
                    .collect::<Vec<_>>();
                let pspecs_ptr = pspecs_array.as_mut_ptr();
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    pspecs.len() as u32,
                    pspecs_ptr,
                )
            }
        }
    }

    fn parent_drag_begin(&self, widget: &Self::Type, context: &gdk::DragContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_begin {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                )
            }
        }
    }

    fn parent_drag_data_delete(&self, widget: &Self::Type, context: &gdk::DragContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_data_delete {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                )
            }
        }
    }

    fn parent_drag_data_get(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_data_get {
                let selection_mut = glib::translate::mut_override(selection_data.to_glib_none().0);
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    selection_mut,
                    info,
                    time,
                )
            }
        }
    }

    fn parent_drag_data_received(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_data_received {
                let selection_mut = glib::translate::mut_override(selection_data.to_glib_none().0);
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    x,
                    y,
                    selection_mut,
                    info,
                    time,
                )
            }
        }
    }

    fn parent_drag_drop(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_drop {
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    x,
                    y,
                    time,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_drag_end(&self, widget: &Self::Type, context: &gdk::DragContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_end {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                )
            }
        }
    }

    fn parent_drag_failed(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        result: DragResult,
    ) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_failed {
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    result.to_glib(),
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_drag_leave(&self, widget: &Self::Type, context: &gdk::DragContext, time: u32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_leave {
                f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    time,
                )
            }
        }
    }

    fn parent_drag_motion(
        &self,
        widget: &Self::Type,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_motion {
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    context.to_glib_none().0,
                    x,
                    y,
                    time,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_draw(&self, widget: &Self::Type, cr: &cairo::Context) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).draw {
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    cr.to_glib_none().0,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_get_request_mode(&self, widget: &Self::Type) -> SizeRequestMode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class).get_request_mode.unwrap();
            from_glib(f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0))
        }
    }

    fn parent_get_preferred_width(&self, widget: &Self::Type) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class).get_preferred_width.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_get_preferred_width_for_height(
        &self,
        widget: &Self::Type,
        height: i32,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class).get_preferred_width_for_height.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                height,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_get_preferred_height(&self, widget: &Self::Type) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class).get_preferred_height.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_get_preferred_height_for_width(&self, widget: &Self::Type, width: i32) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class).get_preferred_height_for_width.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                width,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_size_allocate(&self, widget: &Self::Type, allocation: &Allocation) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .size_allocate
                .expect("No parent class impl for \"size_allocate\"");
            f(
                widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                mut_override(allocation.to_glib_none().0),
            );
        }
    }

    fn parent_realize(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).realize {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }

    fn parent_unrealize(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).unrealize {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }

    fn parent_map(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).map {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }

    fn parent_unmap(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).unmap {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }

    fn parent_motion_notify_event(&self, widget: &Self::Type, event: &gdk::EventMotion) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).motion_notify_event {
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_scroll_event(&self, widget: &Self::Type, event: &gdk::EventScroll) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).scroll_event {
                Inhibit(from_glib(f(
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                )))
            } else {
                Inhibit(false)
            }
        }
    }
}

unsafe impl<T: WidgetImpl> IsSubclassable<T> for Widget {
    fn override_vfuncs(class: &mut ::glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.adjust_baseline_allocation = Some(widget_adjust_baseline_allocation::<T>);
        klass.adjust_baseline_request = Some(widget_adjust_baseline_request::<T>);
        klass.adjust_size_allocation = Some(widget_adjust_size_allocation::<T>);
        klass.adjust_size_request = Some(widget_adjust_size_request::<T>);
        klass.button_press_event = Some(widget_button_press_event::<T>);
        klass.button_release_event = Some(widget_button_release_event::<T>);
        // klass.can_activate_accel = Some(widget_can_activate_accel::<T>);
        klass.child_notify = Some(widget_child_notify::<T>);
        klass.composited_changed = Some(widget_composited_changed::<T>);
        klass.compute_expand = Some(widget_compute_expand::<T>);
        klass.configure_event = Some(widget_configure_event::<T>);
        klass.damage_event = Some(widget_damage_event::<T>);
        klass.delete_event = Some(widget_delete_event::<T>);
        klass.destroy = Some(widget_destroy::<T>);
        klass.destroy_event = Some(widget_destroy_event::<T>);
        klass.direction_changed = Some(widget_direction_changed::<T>);
        klass.dispatch_child_properties_changed =
            Some(widget_dispatch_child_properties_changed::<T>);
        klass.drag_begin = Some(widget_drag_begin::<T>);
        klass.drag_data_delete = Some(widget_drag_data_delete::<T>);
        klass.drag_data_get = Some(widget_drag_data_get::<T>);
        klass.drag_data_received = Some(widget_drag_data_received::<T>);
        klass.drag_drop = Some(widget_drag_drop::<T>);
        klass.drag_end = Some(widget_drag_end::<T>);
        klass.drag_failed = Some(widget_drag_failed::<T>);
        klass.drag_leave = Some(widget_drag_leave::<T>);
        klass.drag_motion = Some(widget_drag_motion::<T>);
        klass.draw = Some(widget_draw::<T>);
        klass.get_request_mode = Some(widget_get_request_mode::<T>);
        klass.get_preferred_width = Some(widget_get_preferred_width::<T>);
        klass.get_preferred_height_for_width = Some(widget_get_preferred_height_for_width::<T>);
        klass.get_preferred_height = Some(widget_get_preferred_height::<T>);
        klass.get_preferred_width_for_height = Some(widget_get_preferred_width_for_height::<T>);
        klass.size_allocate = Some(widget_size_allocate::<T>);
        klass.realize = Some(widget_realize::<T>);
        klass.unrealize = Some(widget_unrealize::<T>);
        klass.map = Some(widget_map::<T>);
        klass.unmap = Some(widget_unmap::<T>);
        klass.motion_notify_event = Some(widget_motion_notify_event::<T>);
        klass.scroll_event = Some(widget_scroll_event::<T>);
    }
}

unsafe extern "C" fn widget_adjust_baseline_allocation<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    baseptr: *mut i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.adjust_baseline_allocation(wrap.unsafe_cast_ref(), &mut *baseptr)
}

unsafe extern "C" fn widget_adjust_baseline_request<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    minptr: *mut i32,
    natptr: *mut i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.adjust_baseline_request(wrap.unsafe_cast_ref(), &mut *minptr, &mut *natptr)
}

unsafe extern "C" fn widget_adjust_size_allocation<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    orientation: gtk_sys::GtkOrientation,
    minptr: *mut i32,
    natptr: *mut i32,
    posptr: *mut i32,
    sizeptr: *mut i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let wrap_orientation: Orientation = from_glib(orientation);

    imp.adjust_size_allocation(
        wrap.unsafe_cast_ref(),
        wrap_orientation,
        &mut *minptr,
        &mut *natptr,
        &mut *posptr,
        &mut *sizeptr,
    )
}

unsafe extern "C" fn widget_adjust_size_request<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    orientation: gtk_sys::GtkOrientation,
    minptr: *mut i32,
    natptr: *mut i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let wrap_orientation: Orientation = from_glib(orientation);

    imp.adjust_size_request(
        wrap.unsafe_cast_ref(),
        wrap_orientation,
        &mut *minptr,
        &mut *natptr,
    )
}

unsafe extern "C" fn widget_button_press_event<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    btnptr: *mut gdk_sys::GdkEventButton,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let evwrap: Borrowed<gdk::EventButton> = from_glib_borrow(btnptr);

    imp.button_press_event(wrap.unsafe_cast_ref(), &evwrap)
        .to_glib()
}

unsafe extern "C" fn widget_button_release_event<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    btnptr: *mut gdk_sys::GdkEventButton,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let evwrap: Borrowed<gdk::EventButton> = from_glib_borrow(btnptr);

    imp.button_release_event(wrap.unsafe_cast_ref(), &evwrap)
        .to_glib()
}

// unsafe extern "C" fn widget_can_activate_accel<T: WidgetImpl>(
//     ptr: *mut gtk_sys::GtkWidget,
//     signal_id: u32,
// ) -> glib_sys::gboolean
// {
//     let instance = &*(ptr as *mut T::Instance);
//     let imp = instance.get_impl();
//     let wrap: Widget = from_glib_borrow(ptr);

//     imp.can_activate_accel(wrap.unsafe_cast_ref(), signal_id) as glib_sys::gboolean
// }

unsafe extern "C" fn widget_child_notify<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    paramptr: *mut gobject_sys::GParamSpec,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let paramwrap: Borrowed<glib::ParamSpec> = from_glib_borrow(paramptr);

    imp.child_notify(wrap.unsafe_cast_ref(), &paramwrap)
}

unsafe extern "C" fn widget_composited_changed<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.composited_changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_compute_expand<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    hexpand_ptr: *mut glib_sys::gboolean,
    vexpand_ptr: *mut glib_sys::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let mut hexpand_p: bool = from_glib(*hexpand_ptr);
    let mut vexpand_p: bool = from_glib(*vexpand_ptr);

    imp.compute_expand(wrap.unsafe_cast_ref(), &mut hexpand_p, &mut vexpand_p);
    *hexpand_ptr = hexpand_p.to_glib();
    *vexpand_ptr = vexpand_p.to_glib();
}

unsafe extern "C" fn widget_configure_event<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    confptr: *mut gdk_sys::GdkEventConfigure,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let evwrap: Borrowed<gdk::EventConfigure> = from_glib_borrow(confptr);

    imp.configure_event(wrap.unsafe_cast_ref(), &evwrap)
        .to_glib()
}

unsafe extern "C" fn widget_damage_event<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    exposeptr: *mut gdk_sys::GdkEventExpose,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let evwrap: Borrowed<gdk::EventExpose> = from_glib_borrow(exposeptr);

    imp.damage_event(wrap.unsafe_cast_ref(), &evwrap).to_glib()
}

unsafe extern "C" fn widget_delete_event<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    anyptr: *mut gdk_sys::GdkEventAny,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let evwrap: Borrowed<gdk::Event> = from_glib_borrow(anyptr);

    imp.delete_event(wrap.unsafe_cast_ref(), &evwrap).to_glib()
}

unsafe extern "C" fn widget_destroy<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.destroy(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn widget_destroy_event<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    anyptr: *mut gdk_sys::GdkEventAny,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let evwrap: Borrowed<gdk::Event> = from_glib_borrow(anyptr);

    imp.destroy_event(wrap.unsafe_cast_ref(), &evwrap).to_glib()
}

unsafe extern "C" fn widget_direction_changed<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    directnptr: gtk_sys::GtkTextDirection,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let dirwrap: TextDirection = from_glib(directnptr);

    imp.direction_changed(wrap.unsafe_cast_ref(), dirwrap)
}

unsafe extern "C" fn widget_dispatch_child_properties_changed<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    n_pspec_ptr: u32,
    pspecsptr: *mut *mut gobject_sys::GParamSpec,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let pspecs: Vec<glib::ParamSpec> =
        FromGlibContainer::from_glib_none_num(pspecsptr, n_pspec_ptr as usize);

    imp.dispatch_child_properties_changed(wrap.unsafe_cast_ref(), &pspecs)
}

unsafe extern "C" fn widget_drag_begin<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

    imp.drag_begin(wrap.unsafe_cast_ref(), &context)
}

unsafe extern "C" fn widget_drag_data_delete<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

    imp.drag_data_delete(wrap.unsafe_cast_ref(), &context)
}

unsafe extern "C" fn widget_drag_data_get<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    selectptr: *mut gtk_sys::GtkSelectionData,
    info: u32,
    time: u32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);
    let selection_data: Borrowed<SelectionData> = from_glib_borrow(selectptr);

    imp.drag_data_get(
        wrap.unsafe_cast_ref(),
        &context,
        &selection_data,
        info,
        time,
    )
}

unsafe extern "C" fn widget_drag_data_received<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    x: i32,
    y: i32,
    selectptr: *mut gtk_sys::GtkSelectionData,
    info: u32,
    time: u32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);
    let selection_data: Borrowed<SelectionData> = from_glib_borrow(selectptr);

    imp.drag_data_received(
        wrap.unsafe_cast_ref(),
        &context,
        x,
        y,
        &selection_data,
        info,
        time,
    )
}

unsafe extern "C" fn widget_drag_drop<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    x: i32,
    y: i32,
    time: u32,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

    imp.drag_drop(wrap.unsafe_cast_ref(), &context, x, y, time)
        .to_glib()
}

unsafe extern "C" fn widget_drag_end<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

    imp.drag_end(wrap.unsafe_cast_ref(), &context)
}

unsafe extern "C" fn widget_drag_failed<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    resultptr: gtk_sys::GtkDragResult,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);
    let result: DragResult = from_glib(resultptr);

    imp.drag_failed(wrap.unsafe_cast_ref(), &context, result)
        .to_glib()
}

unsafe extern "C" fn widget_drag_leave<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    time: u32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

    imp.drag_leave(wrap.unsafe_cast_ref(), &context, time)
}

unsafe extern "C" fn widget_drag_motion<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    x: i32,
    y: i32,
    time: u32,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let context: Borrowed<gdk::DragContext> = from_glib_borrow(ctxptr);

    imp.drag_motion(wrap.unsafe_cast_ref(), &context, x, y, time)
        .to_glib()
}

unsafe extern "C" fn widget_draw<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    cr_ptr: *mut cairo_sys::cairo_t,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let cr: Borrowed<cairo::Context> = from_glib_borrow(cr_ptr);

    imp.draw(wrap.unsafe_cast_ref(), &cr).to_glib()
}

unsafe extern "C" fn widget_get_request_mode<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
) -> gtk_sys::GtkSizeRequestMode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.get_request_mode(wrap.unsafe_cast_ref()).to_glib()
}

unsafe extern "C" fn widget_get_preferred_height<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    minptr: *mut c_int,
    natptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    let (min_size, nat_size) = imp.get_preferred_height(wrap.unsafe_cast_ref());
    if !minptr.is_null() {
        *minptr = min_size;
    }
    if !natptr.is_null() {
        *natptr = nat_size;
    }
}

unsafe extern "C" fn widget_get_preferred_width_for_height<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    height: c_int,
    min_width_ptr: *mut c_int,
    nat_width_ptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    let (min_width, nat_width) = imp.get_preferred_width_for_height(wrap.unsafe_cast_ref(), height);
    if !min_width_ptr.is_null() {
        *min_width_ptr = min_width;
    }
    if !nat_width_ptr.is_null() {
        *nat_width_ptr = nat_width;
    }
}

unsafe extern "C" fn widget_get_preferred_width<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    minptr: *mut c_int,
    natptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let (min_size, nat_size) = imp.get_preferred_width(wrap.unsafe_cast_ref());
    if !minptr.is_null() {
        *minptr = min_size;
    }
    if !natptr.is_null() {
        *natptr = nat_size;
    }
}

unsafe extern "C" fn widget_get_preferred_height_for_width<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    width: c_int,
    min_height_ptr: *mut c_int,
    nat_height_ptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    let (min_height, nat_height) =
        imp.get_preferred_height_for_width(wrap.unsafe_cast_ref(), width);
    if !min_height_ptr.is_null() {
        *min_height_ptr = min_height;
    }
    if !nat_height_ptr.is_null() {
        *nat_height_ptr = nat_height;
    }
}

unsafe extern "C" fn widget_size_allocate<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    allocation: *mut gtk_sys::GtkAllocation,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let allocate: &Allocation = &from_glib_none(allocation);

    imp.size_allocate(wrap.unsafe_cast_ref(), allocate);
}

unsafe extern "C" fn widget_realize<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.realize(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn widget_unrealize<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    imp.unrealize(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn widget_map<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    imp.map(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn widget_unmap<T: WidgetImpl>(ptr: *mut gtk_sys::GtkWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    imp.unmap(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn widget_motion_notify_event<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    mptr: *mut gdk_sys::GdkEventMotion,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let event: Borrowed<gdk::EventMotion> = from_glib_borrow(mptr);

    imp.motion_notify_event(wrap.unsafe_cast_ref(), &event)
        .to_glib()
}

unsafe extern "C" fn widget_scroll_event<T: WidgetImpl>(
    ptr: *mut gtk_sys::GtkWidget,
    mptr: *mut gdk_sys::GdkEventScroll,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);
    let event: Borrowed<gdk::EventScroll> = from_glib_borrow(mptr);

    imp.scroll_event(wrap.unsafe_cast_ref(), &event).to_glib()
}