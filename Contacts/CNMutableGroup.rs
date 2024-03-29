//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNGroup")]
    pub struct CNMutableGroup;

    #[cfg(feature = "Contacts_CNGroup")]
    unsafe impl ClassType for CNMutableGroup {
        #[inherits(NSObject)]
        type Super = CNGroup;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Contacts_CNGroup", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for CNMutableGroup {}

#[cfg(all(feature = "Contacts_CNGroup", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for CNMutableGroup {}

#[cfg(all(feature = "Contacts_CNGroup", feature = "Foundation_NSObject"))]
unsafe impl NSMutableCopying for CNMutableGroup {}

#[cfg(feature = "Contacts_CNGroup")]
unsafe impl NSObjectProtocol for CNMutableGroup {}

#[cfg(all(feature = "Contacts_CNGroup", feature = "Foundation_NSObject"))]
unsafe impl NSSecureCoding for CNMutableGroup {}

extern_methods!(
    #[cfg(feature = "Contacts_CNGroup")]
    unsafe impl CNMutableGroup {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Contacts_CNGroup")]
    unsafe impl CNMutableGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
