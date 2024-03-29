//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILCommunication")]
    pub struct ILMessageCommunication;

    #[cfg(feature = "IdentityLookup_ILCommunication")]
    unsafe impl ClassType for ILMessageCommunication {
        #[inherits(NSObject)]
        type Super = ILCommunication;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "IdentityLookup_ILCommunication"
))]
unsafe impl NSCoding for ILMessageCommunication {}

#[cfg(feature = "IdentityLookup_ILCommunication")]
unsafe impl NSObjectProtocol for ILMessageCommunication {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "IdentityLookup_ILCommunication"
))]
unsafe impl NSSecureCoding for ILMessageCommunication {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILCommunication")]
    unsafe impl ILMessageCommunication {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other messageBody)]
        pub unsafe fn messageBody(&self) -> Option<Id<NSString>>;

        #[method(isEqualToMessageCommunication:)]
        pub unsafe fn isEqualToMessageCommunication(
            &self,
            communication: &ILMessageCommunication,
        ) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "IdentityLookup_ILCommunication")]
    unsafe impl ILMessageCommunication {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
