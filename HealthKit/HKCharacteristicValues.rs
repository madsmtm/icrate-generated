//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKActivityMoveMode(pub NSInteger);
impl HKActivityMoveMode {
    #[doc(alias = "HKActivityMoveModeActiveEnergy")]
    pub const ActiveEnergy: Self = Self(1);
    #[doc(alias = "HKActivityMoveModeAppleMoveTime")]
    pub const AppleMoveTime: Self = Self(2);
}

unsafe impl Encode for HKActivityMoveMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKActivityMoveMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKBiologicalSex(pub NSInteger);
impl HKBiologicalSex {
    #[doc(alias = "HKBiologicalSexNotSet")]
    pub const NotSet: Self = Self(0);
    #[doc(alias = "HKBiologicalSexFemale")]
    pub const Female: Self = Self(1);
    #[doc(alias = "HKBiologicalSexMale")]
    pub const Male: Self = Self(2);
    #[doc(alias = "HKBiologicalSexOther")]
    pub const Other: Self = Self(3);
}

unsafe impl Encode for HKBiologicalSex {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKBiologicalSex {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKBloodType(pub NSInteger);
impl HKBloodType {
    #[doc(alias = "HKBloodTypeNotSet")]
    pub const NotSet: Self = Self(0);
    #[doc(alias = "HKBloodTypeAPositive")]
    pub const APositive: Self = Self(1);
    #[doc(alias = "HKBloodTypeANegative")]
    pub const ANegative: Self = Self(2);
    #[doc(alias = "HKBloodTypeBPositive")]
    pub const BPositive: Self = Self(3);
    #[doc(alias = "HKBloodTypeBNegative")]
    pub const BNegative: Self = Self(4);
    #[doc(alias = "HKBloodTypeABPositive")]
    pub const ABPositive: Self = Self(5);
    #[doc(alias = "HKBloodTypeABNegative")]
    pub const ABNegative: Self = Self(6);
    #[doc(alias = "HKBloodTypeOPositive")]
    pub const OPositive: Self = Self(7);
    #[doc(alias = "HKBloodTypeONegative")]
    pub const ONegative: Self = Self(8);
}

unsafe impl Encode for HKBloodType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKBloodType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKFitzpatrickSkinType(pub NSInteger);
impl HKFitzpatrickSkinType {
    #[doc(alias = "HKFitzpatrickSkinTypeNotSet")]
    pub const NotSet: Self = Self(0);
    #[doc(alias = "HKFitzpatrickSkinTypeI")]
    pub const I: Self = Self(1);
    #[doc(alias = "HKFitzpatrickSkinTypeII")]
    pub const II: Self = Self(2);
    #[doc(alias = "HKFitzpatrickSkinTypeIII")]
    pub const III: Self = Self(3);
    #[doc(alias = "HKFitzpatrickSkinTypeIV")]
    pub const IV: Self = Self(4);
    #[doc(alias = "HKFitzpatrickSkinTypeV")]
    pub const V: Self = Self(5);
    #[doc(alias = "HKFitzpatrickSkinTypeVI")]
    pub const VI: Self = Self(6);
}

unsafe impl Encode for HKFitzpatrickSkinType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKFitzpatrickSkinType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKWheelchairUse(pub NSInteger);
impl HKWheelchairUse {
    #[doc(alias = "HKWheelchairUseNotSet")]
    pub const NotSet: Self = Self(0);
    #[doc(alias = "HKWheelchairUseNo")]
    pub const No: Self = Self(1);
    #[doc(alias = "HKWheelchairUseYes")]
    pub const Yes: Self = Self(2);
}

unsafe impl Encode for HKWheelchairUse {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKWheelchairUse {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
