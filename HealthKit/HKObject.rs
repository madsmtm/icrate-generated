//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKObject;

    unsafe impl ClassType for HKObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for HKObject {}

unsafe impl NSObjectProtocol for HKObject {}

unsafe impl NSSecureCoding for HKObject {}

extern_methods!(
    unsafe impl HKObject {
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        #[cfg(feature = "HKSource")]
        #[deprecated]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Retained<HKSource>;

        #[cfg(feature = "HKSourceRevision")]
        #[method_id(@__retain_semantics Other sourceRevision)]
        pub unsafe fn sourceRevision(&self) -> Retained<HKSourceRevision>;

        #[cfg(feature = "HKDevice")]
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Retained<HKDevice>>;

        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKObject {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static HKPredicateKeyPathUUID: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathSource: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathMetadata: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathCorrelation: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathWorkout: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathDevice: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathSourceRevision: &'static NSString;
}

extern "C" {
    pub static HKPredicateKeyPathWorkoutEffortRelationship: &'static NSString;
}
