//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static UITextAttributeFont: Option<&'static NSString>;
}

extern "C" {
    pub static UITextAttributeTextColor: Option<&'static NSString>;
}

extern "C" {
    pub static UITextAttributeTextShadowColor: Option<&'static NSString>;
}

extern "C" {
    pub static UITextAttributeTextShadowOffset: Option<&'static NSString>;
}

// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UILineBreakMode(pub NSInteger);
impl UILineBreakMode {
    #[deprecated]
    #[doc(alias = "UILineBreakModeWordWrap")]
    pub const WordWrap: Self = Self(0);
    #[deprecated]
    #[doc(alias = "UILineBreakModeCharacterWrap")]
    pub const CharacterWrap: Self = Self(1);
    #[deprecated]
    #[doc(alias = "UILineBreakModeClip")]
    pub const Clip: Self = Self(2);
    #[deprecated]
    #[doc(alias = "UILineBreakModeHeadTruncation")]
    pub const HeadTruncation: Self = Self(3);
    #[deprecated]
    #[doc(alias = "UILineBreakModeTailTruncation")]
    pub const TailTruncation: Self = Self(4);
    #[deprecated]
    #[doc(alias = "UILineBreakModeMiddleTruncation")]
    pub const MiddleTruncation: Self = Self(5);
}

unsafe impl Encode for UILineBreakMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UILineBreakMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextAlignment(pub NSInteger);
impl UITextAlignment {
    #[deprecated]
    #[doc(alias = "UITextAlignmentLeft")]
    pub const Left: Self = Self(0);
    #[deprecated]
    #[doc(alias = "UITextAlignmentCenter")]
    pub const Center: Self = Self(1);
    #[deprecated]
    #[doc(alias = "UITextAlignmentRight")]
    pub const Right: Self = Self(2);
}

unsafe impl Encode for UITextAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIBaselineAdjustment(pub NSInteger);
impl UIBaselineAdjustment {
    #[doc(alias = "UIBaselineAdjustmentAlignBaselines")]
    pub const AlignBaselines: Self = Self(0);
    #[doc(alias = "UIBaselineAdjustmentAlignCenters")]
    pub const AlignCenters: Self = Self(1);
    #[doc(alias = "UIBaselineAdjustmentNone")]
    pub const None: Self = Self(2);
}

unsafe impl Encode for UIBaselineAdjustment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIBaselineAdjustment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}