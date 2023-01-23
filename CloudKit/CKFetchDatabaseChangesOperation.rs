//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchDatabaseChangesOperation")]
    pub struct CKFetchDatabaseChangesOperation;

    #[cfg(feature = "CloudKit_CKFetchDatabaseChangesOperation")]
    unsafe impl ClassType for CKFetchDatabaseChangesOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
    }
);

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchDatabaseChangesOperation")]
    unsafe impl CKFetchDatabaseChangesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method_id(@__retain_semantics Init initWithPreviousServerChangeToken:)]
        pub unsafe fn initWithPreviousServerChangeToken(
            this: Option<Allocated<Self>>,
            previous_server_change_token: Option<&CKServerChangeToken>,
        ) -> Id<Self, Shared>;

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

        #[method(fetchAllChanges)]
        pub unsafe fn fetchAllChanges(&self) -> bool;

        #[method(setFetchAllChanges:)]
        pub unsafe fn setFetchAllChanges(&self, fetch_all_changes: bool);

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(recordZoneWithIDChangedBlock)]
        pub unsafe fn recordZoneWithIDChangedBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordZoneID>,), ()>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(setRecordZoneWithIDChangedBlock:)]
        pub unsafe fn setRecordZoneWithIDChangedBlock(
            &self,
            record_zone_with_id_changed_block: Option<&Block<(NonNull<CKRecordZoneID>,), ()>>,
        );

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(recordZoneWithIDWasDeletedBlock)]
        pub unsafe fn recordZoneWithIDWasDeletedBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordZoneID>,), ()>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(setRecordZoneWithIDWasDeletedBlock:)]
        pub unsafe fn setRecordZoneWithIDWasDeletedBlock(
            &self,
            record_zone_with_id_was_deleted_block: Option<&Block<(NonNull<CKRecordZoneID>,), ()>>,
        );

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(recordZoneWithIDWasPurgedBlock)]
        pub unsafe fn recordZoneWithIDWasPurgedBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordZoneID>,), ()>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(setRecordZoneWithIDWasPurgedBlock:)]
        pub unsafe fn setRecordZoneWithIDWasPurgedBlock(
            &self,
            record_zone_with_id_was_purged_block: Option<&Block<(NonNull<CKRecordZoneID>,), ()>>,
        );

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(recordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock)]
        pub unsafe fn recordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordZoneID>,), ()>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(setRecordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock:)]
        pub unsafe fn setRecordZoneWithIDWasDeletedDueToUserEncryptedDataResetBlock(
            &self,
            record_zone_with_id_was_deleted_due_to_user_encrypted_data_reset_block: Option<
                &Block<(NonNull<CKRecordZoneID>,), ()>,
            >,
        );

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method(changeTokenUpdatedBlock)]
        pub unsafe fn changeTokenUpdatedBlock(
            &self,
        ) -> *mut Block<(NonNull<CKServerChangeToken>,), ()>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method(setChangeTokenUpdatedBlock:)]
        pub unsafe fn setChangeTokenUpdatedBlock(
            &self,
            change_token_updated_block: Option<&Block<(NonNull<CKServerChangeToken>,), ()>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchDatabaseChangesCompletionBlock)]
        pub unsafe fn fetchDatabaseChangesCompletionBlock(
            &self,
        ) -> *mut Block<(*mut CKServerChangeToken, Bool, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSError"
        ))]
        #[method(setFetchDatabaseChangesCompletionBlock:)]
        pub unsafe fn setFetchDatabaseChangesCompletionBlock(
            &self,
            fetch_database_changes_completion_block: Option<
                &Block<(*mut CKServerChangeToken, Bool, *mut NSError), ()>,
            >,
        );
    }
);
