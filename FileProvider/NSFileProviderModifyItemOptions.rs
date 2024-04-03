//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderModifyItemOptions(pub NSUInteger);
impl NSFileProviderModifyItemOptions {
    pub const NSFileProviderModifyItemMayAlreadyExist: Self = Self(1 << 0);
}

unsafe impl Encode for NSFileProviderModifyItemOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderModifyItemOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
