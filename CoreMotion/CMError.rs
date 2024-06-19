//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMError(pub c_uint);
impl CMError {
    #[doc(alias = "CMErrorNULL")]
    pub const NULL: Self = Self(100);
    #[doc(alias = "CMErrorDeviceRequiresMovement")]
    pub const DeviceRequiresMovement: Self = Self(101);
    #[doc(alias = "CMErrorTrueNorthNotAvailable")]
    pub const TrueNorthNotAvailable: Self = Self(102);
    #[doc(alias = "CMErrorUnknown")]
    pub const Unknown: Self = Self(103);
    #[doc(alias = "CMErrorMotionActivityNotAvailable")]
    pub const MotionActivityNotAvailable: Self = Self(104);
    #[doc(alias = "CMErrorMotionActivityNotAuthorized")]
    pub const MotionActivityNotAuthorized: Self = Self(105);
    #[doc(alias = "CMErrorMotionActivityNotEntitled")]
    pub const MotionActivityNotEntitled: Self = Self(106);
    #[doc(alias = "CMErrorInvalidParameter")]
    pub const InvalidParameter: Self = Self(107);
    #[doc(alias = "CMErrorInvalidAction")]
    pub const InvalidAction: Self = Self(108);
    #[doc(alias = "CMErrorNotAvailable")]
    pub const NotAvailable: Self = Self(109);
    #[doc(alias = "CMErrorNotEntitled")]
    pub const NotEntitled: Self = Self(110);
    #[doc(alias = "CMErrorNotAuthorized")]
    pub const NotAuthorized: Self = Self(111);
    #[doc(alias = "CMErrorNilData")]
    pub const NilData: Self = Self(112);
    #[doc(alias = "CMErrorSize")]
    pub const Size: Self = Self(113);
}

unsafe impl Encode for CMError {
    const ENCODING: Encoding = c_uint::ENCODING;
}

unsafe impl RefEncode for CMError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}