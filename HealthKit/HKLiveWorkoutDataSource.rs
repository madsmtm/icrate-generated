//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKLiveWorkoutDataSource;

    unsafe impl ClassType for HKLiveWorkoutDataSource {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for HKLiveWorkoutDataSource {}

extern_methods!(
    unsafe impl HKLiveWorkoutDataSource {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "HealthKit_HKObjectType")]
        #[method_id(@__retain_semantics Other typesToCollect)]
        pub unsafe fn typesToCollect(&self) -> Id<NSSet<HKQuantityType>>;

        #[cfg(all(
            feature = "HealthKit_HKHealthStore",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[method_id(@__retain_semantics Init initWithHealthStore:workoutConfiguration:)]
        pub unsafe fn initWithHealthStore_workoutConfiguration(
            this: Allocated<Self>,
            health_store: &HKHealthStore,
            configuration: Option<&HKWorkoutConfiguration>,
        ) -> Id<Self>;

        #[cfg(feature = "HealthKit_HKObjectType")]
        #[method(enableCollectionForType:predicate:)]
        pub unsafe fn enableCollectionForType_predicate(
            &self,
            quantity_type: &HKQuantityType,
            predicate: Option<&NSPredicate>,
        );

        #[cfg(feature = "HealthKit_HKObjectType")]
        #[method(disableCollectionForType:)]
        pub unsafe fn disableCollectionForType(&self, quantity_type: &HKQuantityType);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKLiveWorkoutDataSource {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
