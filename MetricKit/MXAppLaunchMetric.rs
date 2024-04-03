//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXMetric")]
    pub struct MXAppLaunchMetric;

    #[cfg(feature = "MetricKit_MXMetric")]
    unsafe impl ClassType for MXAppLaunchMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXMetric")]
unsafe impl NSCoding for MXAppLaunchMetric {}

#[cfg(feature = "MetricKit_MXMetric")]
unsafe impl NSObjectProtocol for MXAppLaunchMetric {}

#[cfg(feature = "MetricKit_MXMetric")]
unsafe impl NSSecureCoding for MXAppLaunchMetric {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXMetric")]
    unsafe impl MXAppLaunchMetric {
        #[cfg(feature = "MetricKit_MXHistogram")]
        #[method_id(@__retain_semantics Other histogrammedTimeToFirstDraw)]
        pub unsafe fn histogrammedTimeToFirstDraw(&self) -> Id<MXHistogram<NSUnitDuration>>;

        #[cfg(feature = "MetricKit_MXHistogram")]
        #[method_id(@__retain_semantics Other histogrammedApplicationResumeTime)]
        pub unsafe fn histogrammedApplicationResumeTime(&self) -> Id<MXHistogram<NSUnitDuration>>;

        #[cfg(feature = "MetricKit_MXHistogram")]
        #[method_id(@__retain_semantics Other histogrammedOptimizedTimeToFirstDraw)]
        pub unsafe fn histogrammedOptimizedTimeToFirstDraw(
            &self,
        ) -> Id<MXHistogram<NSUnitDuration>>;

        #[cfg(feature = "MetricKit_MXHistogram")]
        #[method_id(@__retain_semantics Other histogrammedExtendedLaunch)]
        pub unsafe fn histogrammedExtendedLaunch(&self) -> Id<MXHistogram<NSUnitDuration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MetricKit_MXMetric")]
    unsafe impl MXAppLaunchMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
