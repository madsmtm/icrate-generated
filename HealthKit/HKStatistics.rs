//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKStatisticsOptions(pub NSUInteger);
impl HKStatisticsOptions {
    pub const HKStatisticsOptionNone: Self = Self(0);
    pub const HKStatisticsOptionSeparateBySource: Self = Self(1 << 0);
    pub const HKStatisticsOptionDiscreteAverage: Self = Self(1 << 1);
    pub const HKStatisticsOptionDiscreteMin: Self = Self(1 << 2);
    pub const HKStatisticsOptionDiscreteMax: Self = Self(1 << 3);
    pub const HKStatisticsOptionCumulativeSum: Self = Self(1 << 4);
    pub const HKStatisticsOptionMostRecent: Self = Self(1 << 5);
    #[deprecated]
    pub const HKStatisticsOptionDiscreteMostRecent: Self =
        Self(HKStatisticsOptions::HKStatisticsOptionMostRecent.0);
    pub const HKStatisticsOptionDuration: Self = Self(1 << 6);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for HKStatisticsOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for HKStatisticsOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKStatistics;

    unsafe impl ClassType for HKStatistics {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for HKStatistics {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for HKStatistics {}

unsafe impl NSObjectProtocol for HKStatistics {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for HKStatistics {}

extern_methods!(
    unsafe impl HKStatistics {
        #[cfg(feature = "HealthKit_HKObjectType")]
        #[method_id(@__retain_semantics Other quantityType)]
        pub unsafe fn quantityType(&self) -> Id<HKQuantityType>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Id<NSDate>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "HealthKit_HKSource"))]
        #[method_id(@__retain_semantics Other sources)]
        pub unsafe fn sources(&self) -> Option<Id<NSArray<HKSource>>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "HealthKit_HKQuantity", feature = "HealthKit_HKSource"))]
        #[method_id(@__retain_semantics Other averageQuantityForSource:)]
        pub unsafe fn averageQuantityForSource(&self, source: &HKSource) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other averageQuantity)]
        pub unsafe fn averageQuantity(&self) -> Option<Id<HKQuantity>>;

        #[cfg(all(feature = "HealthKit_HKQuantity", feature = "HealthKit_HKSource"))]
        #[method_id(@__retain_semantics Other minimumQuantityForSource:)]
        pub unsafe fn minimumQuantityForSource(&self, source: &HKSource) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other minimumQuantity)]
        pub unsafe fn minimumQuantity(&self) -> Option<Id<HKQuantity>>;

        #[cfg(all(feature = "HealthKit_HKQuantity", feature = "HealthKit_HKSource"))]
        #[method_id(@__retain_semantics Other maximumQuantityForSource:)]
        pub unsafe fn maximumQuantityForSource(&self, source: &HKSource) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other maximumQuantity)]
        pub unsafe fn maximumQuantity(&self) -> Option<Id<HKQuantity>>;

        #[cfg(all(feature = "HealthKit_HKQuantity", feature = "HealthKit_HKSource"))]
        #[method_id(@__retain_semantics Other mostRecentQuantityForSource:)]
        pub unsafe fn mostRecentQuantityForSource(
            &self,
            source: &HKSource,
        ) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other mostRecentQuantity)]
        pub unsafe fn mostRecentQuantity(&self) -> Option<Id<HKQuantity>>;

        #[cfg(all(feature = "Foundation_NSDateInterval", feature = "HealthKit_HKSource"))]
        #[method_id(@__retain_semantics Other mostRecentQuantityDateIntervalForSource:)]
        pub unsafe fn mostRecentQuantityDateIntervalForSource(
            &self,
            source: &HKSource,
        ) -> Option<Id<NSDateInterval>>;

        #[cfg(feature = "Foundation_NSDateInterval")]
        #[method_id(@__retain_semantics Other mostRecentQuantityDateInterval)]
        pub unsafe fn mostRecentQuantityDateInterval(&self) -> Option<Id<NSDateInterval>>;

        #[cfg(all(feature = "HealthKit_HKQuantity", feature = "HealthKit_HKSource"))]
        #[method_id(@__retain_semantics Other sumQuantityForSource:)]
        pub unsafe fn sumQuantityForSource(&self, source: &HKSource) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other sumQuantity)]
        pub unsafe fn sumQuantity(&self) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other duration)]
        pub unsafe fn duration(&self) -> Option<Id<HKQuantity>>;

        #[cfg(all(feature = "HealthKit_HKQuantity", feature = "HealthKit_HKSource"))]
        #[method_id(@__retain_semantics Other durationForSource:)]
        pub unsafe fn durationForSource(&self, source: &HKSource) -> Option<Id<HKQuantity>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKStatistics {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
