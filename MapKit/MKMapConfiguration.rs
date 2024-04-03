//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKMapElevationStyle(pub NSInteger);
impl MKMapElevationStyle {
    #[doc(alias = "MKMapElevationStyleFlat")]
    pub const Flat: Self = Self(0);
    #[doc(alias = "MKMapElevationStyleRealistic")]
    pub const Realistic: Self = Self(1);
}

unsafe impl Encode for MKMapElevationStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MKMapElevationStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapConfiguration;

    unsafe impl ClassType for MKMapConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MKMapConfiguration {}

unsafe impl NSCopying for MKMapConfiguration {}

unsafe impl NSObjectProtocol for MKMapConfiguration {}

unsafe impl NSSecureCoding for MKMapConfiguration {}

extern_methods!(
    unsafe impl MKMapConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(elevationStyle)]
        pub unsafe fn elevationStyle(&self) -> MKMapElevationStyle;

        #[method(setElevationStyle:)]
        pub unsafe fn setElevationStyle(&self, elevation_style: MKMapElevationStyle);
    }
);
