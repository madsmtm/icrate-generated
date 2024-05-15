//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLReshapeFrequencyHint(pub NSInteger);
impl MLReshapeFrequencyHint {
    #[doc(alias = "MLReshapeFrequencyHintFrequent")]
    pub const Frequent: Self = Self(0);
    #[doc(alias = "MLReshapeFrequencyHintInfrequent")]
    pub const Infrequent: Self = Self(1);
}

unsafe impl Encode for MLReshapeFrequencyHint {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MLReshapeFrequencyHint {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
