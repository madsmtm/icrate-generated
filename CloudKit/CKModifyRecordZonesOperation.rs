//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation",
        feature = "Foundation_NSOperation"
    ))]
    pub struct CKModifyRecordZonesOperation;

    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation",
        feature = "Foundation_NSOperation"
    ))]
    unsafe impl ClassType for CKModifyRecordZonesOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
unsafe impl NSObjectProtocol for CKModifyRecordZonesOperation {}

extern_methods!(
    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation",
        feature = "Foundation_NSOperation"
    ))]
    unsafe impl CKModifyRecordZonesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithRecordZonesToSave:recordZoneIDsToDelete:)]
        pub unsafe fn initWithRecordZonesToSave_recordZoneIDsToDelete(
            this: Allocated<Self>,
            record_zones_to_save: Option<&NSArray<CKRecordZone>>,
            record_zone_i_ds_to_delete: Option<&NSArray<CKRecordZoneID>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKRecordZone", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other recordZonesToSave)]
        pub unsafe fn recordZonesToSave(&self) -> Option<Id<NSArray<CKRecordZone>>>;

        #[cfg(all(feature = "CloudKit_CKRecordZone", feature = "Foundation_NSArray"))]
        #[method(setRecordZonesToSave:)]
        pub unsafe fn setRecordZonesToSave(
            &self,
            record_zones_to_save: Option<&NSArray<CKRecordZone>>,
        );

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other recordZoneIDsToDelete)]
        pub unsafe fn recordZoneIDsToDelete(&self) -> Option<Id<NSArray<CKRecordZoneID>>>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method(setRecordZoneIDsToDelete:)]
        pub unsafe fn setRecordZoneIDsToDelete(
            &self,
            record_zone_i_ds_to_delete: Option<&NSArray<CKRecordZoneID>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSError"
        ))]
        #[method(perRecordZoneSaveBlock)]
        pub unsafe fn perRecordZoneSaveBlock(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError)>;

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSError"
        ))]
        #[method(setPerRecordZoneSaveBlock:)]
        pub unsafe fn setPerRecordZoneSaveBlock(
            &self,
            per_record_zone_save_block: Option<
                &Block<dyn Fn(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSError"))]
        #[method(perRecordZoneDeleteBlock)]
        pub unsafe fn perRecordZoneDeleteBlock(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<CKRecordZoneID>, *mut NSError)>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSError"))]
        #[method(setPerRecordZoneDeleteBlock:)]
        pub unsafe fn setPerRecordZoneDeleteBlock(
            &self,
            per_record_zone_delete_block: Option<
                &Block<dyn Fn(NonNull<CKRecordZoneID>, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(modifyRecordZonesCompletionBlock)]
        pub unsafe fn modifyRecordZonesCompletionBlock(
            &self,
        ) -> *mut Block<
            dyn Fn(*mut NSArray<CKRecordZone>, *mut NSArray<CKRecordZoneID>, *mut NSError),
        >;

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(setModifyRecordZonesCompletionBlock:)]
        pub unsafe fn setModifyRecordZonesCompletionBlock(
            &self,
            modify_record_zones_completion_block: Option<
                &Block<
                    dyn Fn(*mut NSArray<CKRecordZone>, *mut NSArray<CKRecordZoneID>, *mut NSError),
                >,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation",
        feature = "Foundation_NSOperation"
    ))]
    unsafe impl CKModifyRecordZonesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
