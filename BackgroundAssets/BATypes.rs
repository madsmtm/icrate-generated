//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BAContentRequest(pub NSInteger);
impl BAContentRequest {
    #[doc(alias = "BAContentRequestInstall")]
    pub const Install: Self = Self(1);
    #[doc(alias = "BAContentRequestUpdate")]
    pub const Update: Self = Self(2);
    #[doc(alias = "BAContentRequestPeriodic")]
    pub const Periodic: Self = Self(3);
}

unsafe impl Encode for BAContentRequest {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for BAContentRequest {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
