//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-contacts")]
use objc2_contacts::*;

use crate::*;

extern_protocol!(
    pub unsafe trait CNContactPickerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "CNContactPicker", feature = "objc2-contacts"))]
        #[optional]
        #[method(contactPicker:didSelectContact:)]
        unsafe fn contactPicker_didSelectContact(
            &self,
            picker: &CNContactPicker,
            contact: &CNContact,
        );

        #[cfg(all(feature = "CNContactPicker", feature = "objc2-contacts"))]
        #[optional]
        #[method(contactPicker:didSelectContactProperty:)]
        unsafe fn contactPicker_didSelectContactProperty(
            &self,
            picker: &CNContactPicker,
            contact_property: &CNContactProperty,
        );

        #[cfg(feature = "CNContactPicker")]
        #[optional]
        #[method(contactPickerWillClose:)]
        unsafe fn contactPickerWillClose(&self, picker: &CNContactPicker);

        #[cfg(feature = "CNContactPicker")]
        #[optional]
        #[method(contactPickerDidClose:)]
        unsafe fn contactPickerDidClose(&self, picker: &CNContactPicker);
    }

    unsafe impl ProtocolType for dyn CNContactPickerDelegate {}
);
