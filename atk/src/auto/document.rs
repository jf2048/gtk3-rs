// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "AtkDocument")]
    pub struct Document(Interface<ffi::AtkDocument, ffi::AtkDocumentIface>);

    match fn {
        type_ => || ffi::atk_document_get_type(),
    }
}

impl Document {
    pub const NONE: Option<&'static Document> = None;
}

pub trait DocumentExt: IsA<Document> + 'static {
    #[doc(alias = "atk_document_get_attribute_value")]
    #[doc(alias = "get_attribute_value")]
    fn attribute_value(&self, attribute_name: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_document_get_attribute_value(
                self.as_ref().to_glib_none().0,
                attribute_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_document_get_current_page_number")]
    #[doc(alias = "get_current_page_number")]
    fn current_page_number(&self) -> i32 {
        unsafe { ffi::atk_document_get_current_page_number(self.as_ref().to_glib_none().0) }
    }

    //#[doc(alias = "atk_document_get_document")]
    //#[doc(alias = "get_document")]
    //fn document(&self) -> /*Unimplemented*/Option<Basic: Pointer> {
    //    unsafe { TODO: call ffi:atk_document_get_document() }
    //}

    #[doc(alias = "atk_document_get_document_type")]
    #[doc(alias = "get_document_type")]
    fn document_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_document_get_document_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_document_get_page_count")]
    #[doc(alias = "get_page_count")]
    fn page_count(&self) -> i32 {
        unsafe { ffi::atk_document_get_page_count(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "atk_document_set_attribute_value")]
    fn set_attribute_value(&self, attribute_name: &str, attribute_value: &str) -> bool {
        unsafe {
            from_glib(ffi::atk_document_set_attribute_value(
                self.as_ref().to_glib_none().0,
                attribute_name.to_glib_none().0,
                attribute_value.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "load-complete")]
    fn connect_load_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn load_complete_trampoline<P: IsA<Document>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkDocument,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Document::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"load-complete\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    load_complete_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "load-stopped")]
    fn connect_load_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn load_stopped_trampoline<P: IsA<Document>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkDocument,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Document::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"load-stopped\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    load_stopped_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-changed")]
    fn connect_page_changed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_changed_trampoline<P: IsA<Document>, F: Fn(&P, i32) + 'static>(
            this: *mut ffi::AtkDocument,
            page_number: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Document::from_glib_borrow(this).unsafe_cast_ref(),
                page_number,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    page_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "reload")]
    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn reload_trampoline<P: IsA<Document>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkDocument,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Document::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"reload\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    reload_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Document>> DocumentExt for O {}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Document")
    }
}
