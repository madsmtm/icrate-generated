//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKQuantityAggregationStyle(pub NSInteger);
impl HKQuantityAggregationStyle {
    #[doc(alias = "HKQuantityAggregationStyleCumulative")]
    pub const Cumulative: Self = Self(0);
    #[doc(alias = "HKQuantityAggregationStyleDiscreteArithmetic")]
    pub const DiscreteArithmetic: Self = Self(1);
    #[deprecated]
    #[doc(alias = "HKQuantityAggregationStyleDiscrete")]
    pub const Discrete: Self = Self(HKQuantityAggregationStyle::DiscreteArithmetic.0);
    #[doc(alias = "HKQuantityAggregationStyleDiscreteTemporallyWeighted")]
    pub const DiscreteTemporallyWeighted: Self = Self(2);
    #[doc(alias = "HKQuantityAggregationStyleDiscreteEquivalentContinuousLevel")]
    pub const DiscreteEquivalentContinuousLevel: Self = Self(3);
}

unsafe impl Encode for HKQuantityAggregationStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKQuantityAggregationStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
