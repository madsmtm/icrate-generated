//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKQuery")]
    pub struct HKSourceQuery;

    #[cfg(feature = "HealthKit_HKQuery")]
    unsafe impl ClassType for HKSourceQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKQuery")]
unsafe impl NSObjectProtocol for HKSourceQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKQuery")]
    unsafe impl HKSourceQuery {
        #[cfg(all(
            feature = "HealthKit_HKObjectType",
            feature = "HealthKit_HKSource",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithSampleType:samplePredicate:completionHandler:)]
        pub unsafe fn initWithSampleType_samplePredicate_completionHandler(
            this: Allocated<Self>,
            sample_type: &HKSampleType,
            object_predicate: Option<&NSPredicate>,
            completion_handler: &Block<
                dyn Fn(NonNull<HKSourceQuery>, *mut NSSet<HKSource>, *mut NSError),
            >,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HealthKit_HKQuery")]
    unsafe impl HKSourceQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKQuery")]
    unsafe impl HKSourceQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
