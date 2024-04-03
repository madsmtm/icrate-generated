//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation"
    ))]
    pub struct CKFetchRecordZonesOperation;

    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation"
    ))]
    unsafe impl ClassType for CKFetchRecordZonesOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKOperation"
))]
unsafe impl NSObjectProtocol for CKFetchRecordZonesOperation {}

extern_methods!(
    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation"
    ))]
    unsafe impl CKFetchRecordZonesOperation {
        #[method_id(@__retain_semantics Other fetchAllRecordZonesOperation)]
        pub unsafe fn fetchAllRecordZonesOperation() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithRecordZoneIDs:)]
        pub unsafe fn initWithRecordZoneIDs(
            this: Allocated<Self>,
            zone_i_ds: &NSArray<CKRecordZoneID>,
        ) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other recordZoneIDs)]
        pub unsafe fn recordZoneIDs(&self) -> Option<Id<NSArray<CKRecordZoneID>>>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(setRecordZoneIDs:)]
        pub unsafe fn setRecordZoneIDs(&self, record_zone_i_ds: Option<&NSArray<CKRecordZoneID>>);

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(perRecordZoneCompletionBlock)]
        pub unsafe fn perRecordZoneCompletionBlock(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError)>;

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(setPerRecordZoneCompletionBlock:)]
        pub unsafe fn setPerRecordZoneCompletionBlock(
            &self,
            per_record_zone_completion_block: Option<
                &Block<dyn Fn(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(fetchRecordZonesCompletionBlock)]
        pub unsafe fn fetchRecordZonesCompletionBlock(
            &self,
        ) -> *mut Block<dyn Fn(*mut NSDictionary<CKRecordZoneID, CKRecordZone>, *mut NSError)>;

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "block2"
        ))]
        #[method(setFetchRecordZonesCompletionBlock:)]
        pub unsafe fn setFetchRecordZonesCompletionBlock(
            &self,
            fetch_record_zones_completion_block: Option<
                &Block<dyn Fn(*mut NSDictionary<CKRecordZoneID, CKRecordZone>, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation"
    ))]
    unsafe impl CKFetchRecordZonesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
