// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtkObjectFactory")]
    pub struct ObjectFactory(Object<ffi::AtkObjectFactory, ffi::AtkObjectFactoryClass>);

    match fn {
        type_ => || ffi::atk_object_factory_get_type(),
    }
}

impl ObjectFactory {
    pub const NONE: Option<&'static ObjectFactory> = None;
}

pub trait ObjectFactoryExt: IsA<ObjectFactory> + 'static {
    #[doc(alias = "atk_object_factory_create_accessible")]
    fn create_accessible(&self, obj: &impl IsA<glib::Object>) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_object_factory_create_accessible(
                self.as_ref().to_glib_none().0,
                obj.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_object_factory_get_accessible_type")]
    #[doc(alias = "get_accessible_type")]
    fn accessible_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::atk_object_factory_get_accessible_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_object_factory_invalidate")]
    fn invalidate(&self) {
        unsafe {
            ffi::atk_object_factory_invalidate(self.as_ref().to_glib_none().0);
        }
    }
}

impl<O: IsA<ObjectFactory>> ObjectFactoryExt for O {}

impl fmt::Display for ObjectFactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ObjectFactory")
    }
}
