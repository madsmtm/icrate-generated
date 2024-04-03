//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static MXErrorDomain: Option<&'static NSErrorDomain>;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MXErrorCode(pub NSInteger);
impl MXErrorCode {
    pub const MXErrorLaunchTaskInvalidID: Self = Self(0);
    pub const MXErrorLaunchTaskMaxCount: Self = Self(1);
    pub const MXErrorLaunchTaskPastDeadline: Self = Self(2);
    pub const MXErrorLaunchTaskDuplicated: Self = Self(3);
    pub const MXErrorLaunchTaskUnknown: Self = Self(4);
    pub const MXErrorLaunchTaskInternalFailure: Self = Self(5);
}

unsafe impl Encode for MXErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MXErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
