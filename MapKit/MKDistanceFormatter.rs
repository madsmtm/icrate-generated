//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKDistanceFormatterUnits(pub NSUInteger);
impl MKDistanceFormatterUnits {
    #[doc(alias = "MKDistanceFormatterUnitsDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MKDistanceFormatterUnitsMetric")]
    pub const Metric: Self = Self(1);
    #[doc(alias = "MKDistanceFormatterUnitsImperial")]
    pub const Imperial: Self = Self(2);
    #[doc(alias = "MKDistanceFormatterUnitsImperialWithYards")]
    pub const ImperialWithYards: Self = Self(3);
}

unsafe impl Encode for MKDistanceFormatterUnits {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MKDistanceFormatterUnits {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKDistanceFormatterUnitStyle(pub NSUInteger);
impl MKDistanceFormatterUnitStyle {
    #[doc(alias = "MKDistanceFormatterUnitStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MKDistanceFormatterUnitStyleAbbreviated")]
    pub const Abbreviated: Self = Self(1);
    #[doc(alias = "MKDistanceFormatterUnitStyleFull")]
    pub const Full: Self = Self(2);
}

unsafe impl Encode for MKDistanceFormatterUnitStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MKDistanceFormatterUnitStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKDistanceFormatter;

    unsafe impl ClassType for MKDistanceFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MKDistanceFormatter {}

unsafe impl NSCopying for MKDistanceFormatter {}

unsafe impl NSObjectProtocol for MKDistanceFormatter {}

extern_methods!(
    unsafe impl MKDistanceFormatter {
        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other stringFromDistance:)]
        pub unsafe fn stringFromDistance(&self, distance: CLLocationDistance) -> Id<NSString>;

        #[cfg(feature = "objc2-core-location")]
        #[method(distanceFromString:)]
        pub unsafe fn distanceFromString(&self, distance: &NSString) -> CLLocationDistance;

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method(units)]
        pub unsafe fn units(&self) -> MKDistanceFormatterUnits;

        #[method(setUnits:)]
        pub unsafe fn setUnits(&self, units: MKDistanceFormatterUnits);

        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> MKDistanceFormatterUnitStyle;

        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unit_style: MKDistanceFormatterUnitStyle);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKDistanceFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
