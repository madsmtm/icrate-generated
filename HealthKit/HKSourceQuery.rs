//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

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
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "Foundation_NSSet",
            feature = "HealthKit_HKObjectType",
            feature = "HealthKit_HKSource"
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
