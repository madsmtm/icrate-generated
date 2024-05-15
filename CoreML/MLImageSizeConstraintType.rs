//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLImageSizeConstraintType(pub NSInteger);
impl MLImageSizeConstraintType {
    #[doc(alias = "MLImageSizeConstraintTypeUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "MLImageSizeConstraintTypeEnumerated")]
    pub const Enumerated: Self = Self(2);
    #[doc(alias = "MLImageSizeConstraintTypeRange")]
    pub const Range: Self = Self(3);
}

unsafe impl Encode for MLImageSizeConstraintType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MLImageSizeConstraintType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
