//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CNPostalAddressFormatterStyle(pub NSInteger);
impl CNPostalAddressFormatterStyle {
    #[doc(alias = "CNPostalAddressFormatterStyleMailingAddress")]
    pub const MailingAddress: Self = Self(0);
}

unsafe impl Encode for CNPostalAddressFormatterStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CNPostalAddressFormatterStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNPostalAddressFormatter;

    unsafe impl ClassType for CNPostalAddressFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CNPostalAddressFormatter {}

unsafe impl NSCopying for CNPostalAddressFormatter {}

unsafe impl NSObjectProtocol for CNPostalAddressFormatter {}

extern_methods!(
    unsafe impl CNPostalAddressFormatter {
        #[cfg(feature = "Contacts_CNPostalAddress")]
        #[method_id(@__retain_semantics Other stringFromPostalAddress:style:)]
        pub unsafe fn stringFromPostalAddress_style(
            postal_address: &CNPostalAddress,
            style: CNPostalAddressFormatterStyle,
        ) -> Id<NSString>;

        #[cfg(feature = "Contacts_CNPostalAddress")]
        #[method_id(@__retain_semantics Other attributedStringFromPostalAddress:style:withDefaultAttributes:)]
        pub unsafe fn attributedStringFromPostalAddress_style_withDefaultAttributes(
            postal_address: &CNPostalAddress,
            style: CNPostalAddressFormatterStyle,
            attributes: &NSDictionary,
        ) -> Id<NSAttributedString>;

        #[method(style)]
        pub unsafe fn style(&self) -> CNPostalAddressFormatterStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: CNPostalAddressFormatterStyle);

        #[cfg(feature = "Contacts_CNPostalAddress")]
        #[method_id(@__retain_semantics Other stringFromPostalAddress:)]
        pub unsafe fn stringFromPostalAddress(
            &self,
            postal_address: &CNPostalAddress,
        ) -> Id<NSString>;

        #[cfg(feature = "Contacts_CNPostalAddress")]
        #[method_id(@__retain_semantics Other attributedStringFromPostalAddress:withDefaultAttributes:)]
        pub unsafe fn attributedStringFromPostalAddress_withDefaultAttributes(
            &self,
            postal_address: &CNPostalAddress,
            attributes: &NSDictionary,
        ) -> Id<NSAttributedString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNPostalAddressFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub static CNPostalAddressPropertyAttribute: &'static NSString;
}

extern "C" {
    pub static CNPostalAddressLocalizedPropertyNameAttribute: &'static NSString;
}
