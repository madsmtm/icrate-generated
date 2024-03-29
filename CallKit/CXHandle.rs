//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CXHandleType(pub NSInteger);
impl CXHandleType {
    #[doc(alias = "CXHandleTypeGeneric")]
    pub const Generic: Self = Self(1);
    #[doc(alias = "CXHandleTypePhoneNumber")]
    pub const PhoneNumber: Self = Self(2);
    #[doc(alias = "CXHandleTypeEmailAddress")]
    pub const EmailAddress: Self = Self(3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CXHandleType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CXHandleType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXHandle;

    unsafe impl ClassType for CXHandle {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CXHandle {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for CXHandle {}

unsafe impl NSObjectProtocol for CXHandle {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CXHandle {}

extern_methods!(
    unsafe impl CXHandle {
        #[method(type)]
        pub unsafe fn r#type(&self) -> CXHandleType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithType:value:)]
        pub unsafe fn initWithType_value(
            this: Allocated<Self>,
            r#type: CXHandleType,
            value: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method(isEqualToHandle:)]
        pub unsafe fn isEqualToHandle(&self, handle: &CXHandle) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CXHandle {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
