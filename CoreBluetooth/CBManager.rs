//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CBManagerState(pub NSInteger);
impl CBManagerState {
    #[doc(alias = "CBManagerStateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CBManagerStateResetting")]
    pub const Resetting: Self = Self(1);
    #[doc(alias = "CBManagerStateUnsupported")]
    pub const Unsupported: Self = Self(2);
    #[doc(alias = "CBManagerStateUnauthorized")]
    pub const Unauthorized: Self = Self(3);
    #[doc(alias = "CBManagerStatePoweredOff")]
    pub const PoweredOff: Self = Self(4);
    #[doc(alias = "CBManagerStatePoweredOn")]
    pub const PoweredOn: Self = Self(5);
}

unsafe impl Encode for CBManagerState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CBManagerState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CBManagerAuthorization(pub NSInteger);
impl CBManagerAuthorization {
    #[doc(alias = "CBManagerAuthorizationNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "CBManagerAuthorizationRestricted")]
    pub const Restricted: Self = Self(1);
    #[doc(alias = "CBManagerAuthorizationDenied")]
    pub const Denied: Self = Self(2);
    #[doc(alias = "CBManagerAuthorizationAllowedAlways")]
    pub const AllowedAlways: Self = Self(3);
}

unsafe impl Encode for CBManagerAuthorization {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CBManagerAuthorization {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CBManager;

    unsafe impl ClassType for CBManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CBManager {}

extern_methods!(
    unsafe impl CBManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method(state)]
        pub unsafe fn state(&self) -> CBManagerState;

        #[deprecated]
        #[method(authorization)]
        pub unsafe fn authorization(&self) -> CBManagerAuthorization;

        #[method(authorization)]
        pub unsafe fn authorization_class() -> CBManagerAuthorization;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CBManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
