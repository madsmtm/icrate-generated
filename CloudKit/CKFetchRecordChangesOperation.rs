//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchRecordChangesOperation")]
    #[deprecated]
    pub struct CKFetchRecordChangesOperation;

    #[cfg(feature = "CloudKit_CKFetchRecordChangesOperation")]
    unsafe impl ClassType for CKFetchRecordChangesOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
    }
);

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchRecordChangesOperation")]
    unsafe impl CKFetchRecordChangesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "CloudKit_CKRecordZoneID",
            feature = "CloudKit_CKServerChangeToken"
        ))]
        #[method_id(@__retain_semantics Init initWithRecordZoneID:previousServerChangeToken:)]
        pub unsafe fn initWithRecordZoneID_previousServerChangeToken(
            this: Option<Allocated<Self>>,
            record_zone_id: &CKRecordZoneID,
            previous_server_change_token: Option<&CKServerChangeToken>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other recordZoneID)]
        pub unsafe fn recordZoneID(&self) -> Option<Id<CKRecordZoneID, Shared>>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(setRecordZoneID:)]
        pub unsafe fn setRecordZoneID(&self, record_zone_id: Option<&CKRecordZoneID>);

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method_id(@__retain_semantics Other previousServerChangeToken)]
        pub unsafe fn previousServerChangeToken(&self) -> Option<Id<CKServerChangeToken, Shared>>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method(setPreviousServerChangeToken:)]
        pub unsafe fn setPreviousServerChangeToken(
            &self,
            previous_server_change_token: Option<&CKServerChangeToken>,
        );

        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Id<NSArray<CKRecordFieldKey>, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);

        #[cfg(feature = "CloudKit_CKRecord")]
        #[method(recordChangedBlock)]
        pub unsafe fn recordChangedBlock(&self) -> *mut Block<(NonNull<CKRecord>,), ()>;

        #[cfg(feature = "CloudKit_CKRecord")]
        #[method(setRecordChangedBlock:)]
        pub unsafe fn setRecordChangedBlock(
            &self,
            record_changed_block: Option<&Block<(NonNull<CKRecord>,), ()>>,
        );

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method(recordWithIDWasDeletedBlock)]
        pub unsafe fn recordWithIDWasDeletedBlock(&self) -> *mut Block<(NonNull<CKRecordID>,), ()>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method(setRecordWithIDWasDeletedBlock:)]
        pub unsafe fn setRecordWithIDWasDeletedBlock(
            &self,
            record_with_id_was_deleted_block: Option<&Block<(NonNull<CKRecordID>,), ()>>,
        );

        #[method(moreComing)]
        pub unsafe fn moreComing(&self) -> bool;

        #[cfg(all(
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchRecordChangesCompletionBlock)]
        pub unsafe fn fetchRecordChangesCompletionBlock(
            &self,
        ) -> *mut Block<(*mut CKServerChangeToken, *mut NSData, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError"
        ))]
        #[method(setFetchRecordChangesCompletionBlock:)]
        pub unsafe fn setFetchRecordChangesCompletionBlock(
            &self,
            fetch_record_changes_completion_block: Option<
                &Block<(*mut CKServerChangeToken, *mut NSData, *mut NSError), ()>,
            >,
        );
    }
);
