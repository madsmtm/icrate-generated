//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXMetric")]
    pub struct MXAnimationMetric;

    #[cfg(feature = "MetricKit_MXMetric")]
    unsafe impl ClassType for MXAnimationMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Foundation_NSObject", feature = "MetricKit_MXMetric"))]
unsafe impl NSCoding for MXAnimationMetric {}

#[cfg(feature = "MetricKit_MXMetric")]
unsafe impl NSObjectProtocol for MXAnimationMetric {}

#[cfg(all(feature = "Foundation_NSObject", feature = "MetricKit_MXMetric"))]
unsafe impl NSSecureCoding for MXAnimationMetric {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXMetric")]
    unsafe impl MXAnimationMetric {
        #[cfg(all(feature = "Foundation_NSMeasurement", feature = "Foundation_NSUnit"))]
        #[method_id(@__retain_semantics Other scrollHitchTimeRatio)]
        pub unsafe fn scrollHitchTimeRatio(&self) -> Id<NSMeasurement<NSUnit>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MetricKit_MXMetric")]
    unsafe impl MXAnimationMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
