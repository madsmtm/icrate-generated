//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEMessageSigner;

    unsafe impl ClassType for MEMessageSigner {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for MEMessageSigner {}

unsafe impl NSObjectProtocol for MEMessageSigner {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for MEMessageSigner {}

extern_methods!(
    unsafe impl MEMessageSigner {
        #[cfg(all(feature = "Foundation_NSArray", feature = "MailKit_MEEmailAddress"))]
        #[method_id(@__retain_semantics Other emailAddresses)]
        pub unsafe fn emailAddresses(&self) -> Id<NSArray<MEEmailAddress>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Id<NSData>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSString",
            feature = "MailKit_MEEmailAddress"
        ))]
        #[method_id(@__retain_semantics Init initWithEmailAddresses:signatureLabel:context:)]
        pub unsafe fn initWithEmailAddresses_signatureLabel_context(
            this: Allocated<Self>,
            email_addresses: &NSArray<MEEmailAddress>,
            label: &NSString,
            context: Option<&NSData>,
        ) -> Id<Self>;
    }
);
