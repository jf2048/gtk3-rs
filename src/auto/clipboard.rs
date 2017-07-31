// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use SelectionData;
use TextBuffer;
use ffi;
use gdk;
use gdk_pixbuf;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Clipboard(Object<ffi::GtkClipboard>);

    match fn {
        get_type => || ffi::gtk_clipboard_get_type(),
    }
}

impl Clipboard {
    pub fn get(selection: &gdk::Atom) -> Clipboard {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get(selection.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_default(display: &gdk::Display) -> Option<Clipboard> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_default(display.to_glib_none().0))
        }
    }

    pub fn get_for_display(display: &gdk::Display, selection: &gdk::Atom) -> Clipboard {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_for_display(display.to_glib_none().0, selection.to_glib_none().0))
        }
    }
}

pub trait ClipboardExt {
    fn clear(&self);

    fn get_display(&self) -> Option<gdk::Display>;

    fn get_owner(&self) -> Option<glib::Object>;

    #[cfg(feature = "v3_22")]
    fn get_selection(&self) -> Option<gdk::Atom>;

    //fn request_contents<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, target: &gdk::Atom, callback: /*Unknown conversion*//*Unimplemented*/ClipboardReceivedFunc, user_data: P);

    //fn request_image<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardImageReceivedFunc, user_data: P);

    //fn request_rich_text<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, buffer: &TextBuffer, callback: /*Unknown conversion*//*Unimplemented*/ClipboardRichTextReceivedFunc, user_data: P);

    //fn request_targets<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardTargetsReceivedFunc, user_data: P);

    //fn request_text<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardTextReceivedFunc, user_data: P);

    //fn request_uris<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardURIReceivedFunc, user_data: P);

    fn set_image(&self, pixbuf: &gdk_pixbuf::Pixbuf);

    fn set_text(&self, text: &str);

    //fn set_with_data<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, targets: &[&TargetEntry], get_func: /*Unknown conversion*//*Unimplemented*/ClipboardGetFunc, clear_func: /*Unknown conversion*//*Unimplemented*/ClipboardClearFunc, user_data: P) -> bool;

    //fn set_with_owner<P: IsA<glib::Object>>(&self, targets: &[&TargetEntry], get_func: /*Unknown conversion*//*Unimplemented*/ClipboardGetFunc, clear_func: /*Unknown conversion*//*Unimplemented*/ClipboardClearFunc, owner: &P) -> bool;

    fn store(&self);

    fn wait_for_contents(&self, target: &gdk::Atom) -> Option<SelectionData>;

    fn wait_for_image(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn wait_for_rich_text(&self, buffer: &TextBuffer) -> (Vec<u8>, gdk::Atom);

    fn wait_for_targets(&self) -> Option<Vec<gdk::Atom>>;

    fn wait_for_text(&self) -> Option<String>;

    fn wait_for_uris(&self) -> Vec<String>;

    fn wait_is_image_available(&self) -> bool;

    fn wait_is_rich_text_available(&self, buffer: &TextBuffer) -> bool;

    fn wait_is_target_available(&self, target: &gdk::Atom) -> bool;

    fn wait_is_text_available(&self) -> bool;

    fn wait_is_uris_available(&self) -> bool;

    //fn connect_owner_change<Unsupported or ignored types>(&self, f: F) -> u64;
}

impl<O: IsA<Clipboard>> ClipboardExt for O {
    fn clear(&self) {
        unsafe {
            ffi::gtk_clipboard_clear(self.to_glib_none().0);
        }
    }

    fn get_display(&self) -> Option<gdk::Display> {
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_display(self.to_glib_none().0))
        }
    }

    fn get_owner(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_owner(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_selection(&self) -> Option<gdk::Atom> {
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_selection(self.to_glib_none().0))
        }
    }

    //fn request_contents<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, target: &gdk::Atom, callback: /*Unknown conversion*//*Unimplemented*/ClipboardReceivedFunc, user_data: P) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_contents() }
    //}

    //fn request_image<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardImageReceivedFunc, user_data: P) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_image() }
    //}

    //fn request_rich_text<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, buffer: &TextBuffer, callback: /*Unknown conversion*//*Unimplemented*/ClipboardRichTextReceivedFunc, user_data: P) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_rich_text() }
    //}

    //fn request_targets<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardTargetsReceivedFunc, user_data: P) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_targets() }
    //}

    //fn request_text<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardTextReceivedFunc, user_data: P) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_text() }
    //}

    //fn request_uris<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardURIReceivedFunc, user_data: P) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_uris() }
    //}

    fn set_image(&self, pixbuf: &gdk_pixbuf::Pixbuf) {
        unsafe {
            ffi::gtk_clipboard_set_image(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    fn set_text(&self, text: &str) {
        let len = text.len() as i32;
        unsafe {
            ffi::gtk_clipboard_set_text(self.to_glib_none().0, text.to_glib_none().0, len);
        }
    }

    //fn set_with_data<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, targets: &[&TargetEntry], get_func: /*Unknown conversion*//*Unimplemented*/ClipboardGetFunc, clear_func: /*Unknown conversion*//*Unimplemented*/ClipboardClearFunc, user_data: P) -> bool {
    //    unsafe { TODO: call ffi::gtk_clipboard_set_with_data() }
    //}

    //fn set_with_owner<P: IsA<glib::Object>>(&self, targets: &[&TargetEntry], get_func: /*Unknown conversion*//*Unimplemented*/ClipboardGetFunc, clear_func: /*Unknown conversion*//*Unimplemented*/ClipboardClearFunc, owner: &P) -> bool {
    //    unsafe { TODO: call ffi::gtk_clipboard_set_with_owner() }
    //}

    fn store(&self) {
        unsafe {
            ffi::gtk_clipboard_store(self.to_glib_none().0);
        }
    }

    fn wait_for_contents(&self, target: &gdk::Atom) -> Option<SelectionData> {
        unsafe {
            from_glib_full(ffi::gtk_clipboard_wait_for_contents(self.to_glib_none().0, target.to_glib_none().0))
        }
    }

    fn wait_for_image(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_full(ffi::gtk_clipboard_wait_for_image(self.to_glib_none().0))
        }
    }

    fn wait_for_rich_text(&self, buffer: &TextBuffer) -> (Vec<u8>, gdk::Atom) {
        unsafe {
            let mut format = gdk::Atom::uninitialized();
            let mut length = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_full_num(ffi::gtk_clipboard_wait_for_rich_text(self.to_glib_none().0, buffer.to_glib_none().0, format.to_glib_none_mut().0, &mut length), length as usize);
            (ret, format)
        }
    }

    fn wait_for_targets(&self) -> Option<Vec<gdk::Atom>> {
        unsafe {
            let mut targets = ptr::null_mut();
            let mut n_targets = mem::uninitialized();
            let ret = from_glib(ffi::gtk_clipboard_wait_for_targets(self.to_glib_none().0, &mut targets, &mut n_targets));
            if ret { Some(FromGlibContainer::from_glib_container_num(targets, n_targets as usize)) } else { None }
        }
    }

    fn wait_for_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_clipboard_wait_for_text(self.to_glib_none().0))
        }
    }

    fn wait_for_uris(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_clipboard_wait_for_uris(self.to_glib_none().0))
        }
    }

    fn wait_is_image_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_image_available(self.to_glib_none().0))
        }
    }

    fn wait_is_rich_text_available(&self, buffer: &TextBuffer) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_rich_text_available(self.to_glib_none().0, buffer.to_glib_none().0))
        }
    }

    fn wait_is_target_available(&self, target: &gdk::Atom) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_target_available(self.to_glib_none().0, target.to_glib_none().0))
        }
    }

    fn wait_is_text_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_text_available(self.to_glib_none().0))
        }
    }

    fn wait_is_uris_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_uris_available(self.to_glib_none().0))
        }
    }

    //fn connect_owner_change<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored event: Gdk.EventOwnerChange
    //}
}
