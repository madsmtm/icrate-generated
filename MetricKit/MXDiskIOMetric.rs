//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXMetric")]
    pub struct MXDiskIOMetric;

    #[cfg(feature = "MetricKit_MXMetric")]
    unsafe impl ClassType for MXDiskIOMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXMetric")]
unsafe impl NSCoding for MXDiskIOMetric {}

#[cfg(feature = "MetricKit_MXMetric")]
unsafe impl NSObjectProtocol for MXDiskIOMetric {}

#[cfg(feature = "MetricKit_MXMetric")]
unsafe impl NSSecureCoding for MXDiskIOMetric {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXMetric")]
    unsafe impl MXDiskIOMetric {
        #[method_id(@__retain_semantics Other cumulativeLogicalWrites)]
        pub unsafe fn cumulativeLogicalWrites(&self)
            -> Id<NSMeasurement<NSUnitInformationStorage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MetricKit_MXMetric")]
    unsafe impl MXDiskIOMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
