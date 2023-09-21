//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSValidatedUserInterfaceItem: IsMainThreadOnly {
        #[method(action)]
        unsafe fn action(&self) -> Option<Sel>;

        #[method(tag)]
        unsafe fn tag(&self) -> NSInteger;
    }

    unsafe impl ProtocolType for dyn NSValidatedUserInterfaceItem {}
);

extern_protocol!(
    pub unsafe trait NSUserInterfaceValidations: IsMainThreadOnly {
        #[method(validateUserInterfaceItem:)]
        unsafe fn validateUserInterfaceItem(
            &self,
            item: &(impl NSValidatedUserInterfaceItem + Message),
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSUserInterfaceValidations {}
);
