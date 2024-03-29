// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `CloudKit` framework
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "CloudKit", kind = "framework")]
extern "C" {}

#[cfg(feature = "CloudKit_CKAcceptSharesOperation")]
#[path = "CKAcceptSharesOperation.rs"]
mod __CKAcceptSharesOperation;
#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
#[path = "CKAllowedSharingOptions.rs"]
mod __CKAllowedSharingOptions;
#[cfg(feature = "CloudKit_CKAsset")]
#[path = "CKAsset.rs"]
mod __CKAsset;
#[cfg(feature = "CloudKit_CKContainer")]
#[path = "CKContainer.rs"]
mod __CKContainer;
#[cfg(feature = "CloudKit_CKDatabase")]
#[path = "CKDatabase.rs"]
mod __CKDatabase;
#[cfg(feature = "CloudKit_CKDatabaseOperation")]
#[path = "CKDatabaseOperation.rs"]
mod __CKDatabaseOperation;
#[cfg(feature = "CloudKit_CKDefines")]
#[path = "CKDefines.rs"]
mod __CKDefines;
#[cfg(feature = "CloudKit_CKDiscoverAllUserIdentitiesOperation")]
#[path = "CKDiscoverAllUserIdentitiesOperation.rs"]
mod __CKDiscoverAllUserIdentitiesOperation;
#[cfg(feature = "CloudKit_CKDiscoverUserIdentitiesOperation")]
#[path = "CKDiscoverUserIdentitiesOperation.rs"]
mod __CKDiscoverUserIdentitiesOperation;
#[cfg(feature = "CloudKit_CKError")]
#[path = "CKError.rs"]
mod __CKError;
#[cfg(feature = "CloudKit_CKFetchDatabaseChangesOperation")]
#[path = "CKFetchDatabaseChangesOperation.rs"]
mod __CKFetchDatabaseChangesOperation;
#[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
#[path = "CKFetchNotificationChangesOperation.rs"]
mod __CKFetchNotificationChangesOperation;
#[cfg(feature = "CloudKit_CKFetchRecordChangesOperation")]
#[path = "CKFetchRecordChangesOperation.rs"]
mod __CKFetchRecordChangesOperation;
#[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOperation")]
#[path = "CKFetchRecordZoneChangesOperation.rs"]
mod __CKFetchRecordZoneChangesOperation;
#[cfg(feature = "CloudKit_CKFetchRecordZonesOperation")]
#[path = "CKFetchRecordZonesOperation.rs"]
mod __CKFetchRecordZonesOperation;
#[cfg(feature = "CloudKit_CKFetchRecordsOperation")]
#[path = "CKFetchRecordsOperation.rs"]
mod __CKFetchRecordsOperation;
#[cfg(feature = "CloudKit_CKFetchShareMetadataOperation")]
#[path = "CKFetchShareMetadataOperation.rs"]
mod __CKFetchShareMetadataOperation;
#[cfg(feature = "CloudKit_CKFetchShareParticipantsOperation")]
#[path = "CKFetchShareParticipantsOperation.rs"]
mod __CKFetchShareParticipantsOperation;
#[cfg(feature = "CloudKit_CKFetchSubscriptionsOperation")]
#[path = "CKFetchSubscriptionsOperation.rs"]
mod __CKFetchSubscriptionsOperation;
#[cfg(feature = "CloudKit_CKFetchWebAuthTokenOperation")]
#[path = "CKFetchWebAuthTokenOperation.rs"]
mod __CKFetchWebAuthTokenOperation;
#[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
#[path = "CKLocationSortDescriptor.rs"]
mod __CKLocationSortDescriptor;
#[cfg(feature = "CloudKit_CKMarkNotificationsReadOperation")]
#[path = "CKMarkNotificationsReadOperation.rs"]
mod __CKMarkNotificationsReadOperation;
#[cfg(feature = "CloudKit_CKModifyBadgeOperation")]
#[path = "CKModifyBadgeOperation.rs"]
mod __CKModifyBadgeOperation;
#[cfg(feature = "CloudKit_CKModifyRecordZonesOperation")]
#[path = "CKModifyRecordZonesOperation.rs"]
mod __CKModifyRecordZonesOperation;
#[cfg(feature = "CloudKit_CKModifyRecordsOperation")]
#[path = "CKModifyRecordsOperation.rs"]
mod __CKModifyRecordsOperation;
#[cfg(feature = "CloudKit_CKModifySubscriptionsOperation")]
#[path = "CKModifySubscriptionsOperation.rs"]
mod __CKModifySubscriptionsOperation;
#[cfg(feature = "CloudKit_CKNotification")]
#[path = "CKNotification.rs"]
mod __CKNotification;
#[cfg(feature = "CloudKit_CKOperation")]
#[path = "CKOperation.rs"]
mod __CKOperation;
#[cfg(feature = "CloudKit_CKOperationGroup")]
#[path = "CKOperationGroup.rs"]
mod __CKOperationGroup;
#[cfg(feature = "CloudKit_CKQuery")]
#[path = "CKQuery.rs"]
mod __CKQuery;
#[cfg(feature = "CloudKit_CKQueryOperation")]
#[path = "CKQueryOperation.rs"]
mod __CKQueryOperation;
#[cfg(feature = "CloudKit_CKRecord")]
#[path = "CKRecord.rs"]
mod __CKRecord;
#[cfg(feature = "CloudKit_CKRecordID")]
#[path = "CKRecordID.rs"]
mod __CKRecordID;
#[cfg(feature = "CloudKit_CKRecordZone")]
#[path = "CKRecordZone.rs"]
mod __CKRecordZone;
#[cfg(feature = "CloudKit_CKRecordZoneID")]
#[path = "CKRecordZoneID.rs"]
mod __CKRecordZoneID;
#[cfg(feature = "CloudKit_CKReference")]
#[path = "CKReference.rs"]
mod __CKReference;
#[cfg(feature = "CloudKit_CKServerChangeToken")]
#[path = "CKServerChangeToken.rs"]
mod __CKServerChangeToken;
#[cfg(feature = "CloudKit_CKShare")]
#[path = "CKShare.rs"]
mod __CKShare;
#[cfg(feature = "CloudKit_CKShareMetadata")]
#[path = "CKShareMetadata.rs"]
mod __CKShareMetadata;
#[cfg(feature = "CloudKit_CKShareParticipant")]
#[path = "CKShareParticipant.rs"]
mod __CKShareParticipant;
#[cfg(feature = "CloudKit_CKSubscription")]
#[path = "CKSubscription.rs"]
mod __CKSubscription;
#[cfg(feature = "CloudKit_CKSyncEngine")]
#[path = "CKSyncEngine.rs"]
mod __CKSyncEngine;
#[cfg(feature = "CloudKit_CKSyncEngineConfiguration")]
#[path = "CKSyncEngineConfiguration.rs"]
mod __CKSyncEngineConfiguration;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
#[path = "CKSyncEngineEvent.rs"]
mod __CKSyncEngineEvent;
#[cfg(feature = "CloudKit_CKSyncEngineRecordZoneChangeBatch")]
#[path = "CKSyncEngineRecordZoneChangeBatch.rs"]
mod __CKSyncEngineRecordZoneChangeBatch;
#[cfg(feature = "CloudKit_CKSyncEngineState")]
#[path = "CKSyncEngineState.rs"]
mod __CKSyncEngineState;
#[cfg(feature = "CloudKit_CKSystemSharingUIObserver")]
#[path = "CKSystemSharingUIObserver.rs"]
mod __CKSystemSharingUIObserver;
#[cfg(feature = "CloudKit_CKUserIdentity")]
#[path = "CKUserIdentity.rs"]
mod __CKUserIdentity;
#[cfg(feature = "CloudKit_CKUserIdentityLookupInfo")]
#[path = "CKUserIdentityLookupInfo.rs"]
mod __CKUserIdentityLookupInfo;
#[cfg(feature = "CloudKit_NSItemProvider_CKSharingSupport")]
#[path = "NSItemProvider_CKSharingSupport.rs"]
mod __NSItemProvider_CKSharingSupport;

#[cfg(all(
    feature = "CloudKit_CKAcceptSharesOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKAcceptSharesOperation::CKAcceptSharesOperation;
#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
pub use self::__CKAllowedSharingOptions::CKAllowedSharingOptions;
#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
pub use self::__CKAllowedSharingOptions::CKSharingParticipantAccessOption;
#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
pub use self::__CKAllowedSharingOptions::CKSharingParticipantPermissionOption;
#[cfg(feature = "CloudKit_CKAsset")]
pub use self::__CKAsset::CKAsset;
#[cfg(all(feature = "CloudKit_CKContainer", feature = "Foundation_NSString"))]
pub use self::__CKContainer::CKAccountChangedNotification;
#[cfg(feature = "CloudKit_CKContainer")]
pub use self::__CKContainer::CKAccountStatus;
#[cfg(all(feature = "CloudKit_CKContainer", feature = "Foundation_NSError"))]
pub use self::__CKContainer::CKApplicationPermissionBlock;
#[cfg(feature = "CloudKit_CKContainer")]
pub use self::__CKContainer::CKApplicationPermissionStatus;
#[cfg(feature = "CloudKit_CKContainer")]
pub use self::__CKContainer::CKApplicationPermissions;
#[cfg(feature = "CloudKit_CKContainer")]
pub use self::__CKContainer::CKContainer;
#[cfg(all(feature = "CloudKit_CKContainer", feature = "Foundation_NSString"))]
pub use self::__CKContainer::CKCurrentUserDefaultName;
#[cfg(all(feature = "CloudKit_CKContainer", feature = "Foundation_NSString"))]
pub use self::__CKContainer::CKOwnerDefaultName;
#[cfg(feature = "CloudKit_CKDatabase")]
pub use self::__CKDatabase::CKDatabase;
#[cfg(feature = "CloudKit_CKDatabase")]
pub use self::__CKDatabase::CKDatabaseScope;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKDatabaseOperation::CKDatabaseOperation;
#[cfg(all(
    feature = "CloudKit_CKDiscoverAllUserIdentitiesOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKDiscoverAllUserIdentitiesOperation::CKDiscoverAllUserIdentitiesOperation;
#[cfg(all(
    feature = "CloudKit_CKDiscoverUserIdentitiesOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKDiscoverUserIdentitiesOperation::CKDiscoverUserIdentitiesOperation;
#[cfg(feature = "CloudKit_CKError")]
pub use self::__CKError::CKErrorCode;
#[cfg(all(feature = "CloudKit_CKError", feature = "Foundation_NSString"))]
pub use self::__CKError::CKErrorDomain;
#[cfg(all(feature = "CloudKit_CKError", feature = "Foundation_NSString"))]
pub use self::__CKError::CKErrorRetryAfterKey;
#[cfg(all(feature = "CloudKit_CKError", feature = "Foundation_NSString"))]
pub use self::__CKError::CKErrorUserDidResetEncryptedDataKey;
#[cfg(all(feature = "CloudKit_CKError", feature = "Foundation_NSString"))]
pub use self::__CKError::CKPartialErrorsByItemIDKey;
#[cfg(all(feature = "CloudKit_CKError", feature = "Foundation_NSString"))]
pub use self::__CKError::CKRecordChangedErrorAncestorRecordKey;
#[cfg(all(feature = "CloudKit_CKError", feature = "Foundation_NSString"))]
pub use self::__CKError::CKRecordChangedErrorClientRecordKey;
#[cfg(all(feature = "CloudKit_CKError", feature = "Foundation_NSString"))]
pub use self::__CKError::CKRecordChangedErrorServerRecordKey;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKFetchDatabaseChangesOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKFetchDatabaseChangesOperation::CKFetchDatabaseChangesOperation;
#[cfg(all(
    feature = "CloudKit_CKFetchNotificationChangesOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKFetchNotificationChangesOperation::CKFetchNotificationChangesOperation;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKFetchRecordChangesOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKFetchRecordChangesOperation::CKFetchRecordChangesOperation;
#[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOperation")]
pub use self::__CKFetchRecordZoneChangesOperation::CKFetchRecordZoneChangesConfiguration;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKFetchRecordZoneChangesOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKFetchRecordZoneChangesOperation::CKFetchRecordZoneChangesOperation;
#[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOperation")]
pub use self::__CKFetchRecordZoneChangesOperation::CKFetchRecordZoneChangesOptions;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKFetchRecordZonesOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKFetchRecordZonesOperation::CKFetchRecordZonesOperation;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKFetchRecordsOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKFetchRecordsOperation::CKFetchRecordsOperation;
#[cfg(all(
    feature = "CloudKit_CKFetchShareMetadataOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKFetchShareMetadataOperation::CKFetchShareMetadataOperation;
#[cfg(all(
    feature = "CloudKit_CKFetchShareParticipantsOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKFetchShareParticipantsOperation::CKFetchShareParticipantsOperation;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKFetchSubscriptionsOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKFetchSubscriptionsOperation::CKFetchSubscriptionsOperation;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKFetchWebAuthTokenOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKFetchWebAuthTokenOperation::CKFetchWebAuthTokenOperation;
#[cfg(all(
    feature = "CloudKit_CKLocationSortDescriptor",
    feature = "Foundation_NSSortDescriptor"
))]
pub use self::__CKLocationSortDescriptor::CKLocationSortDescriptor;
#[cfg(all(
    feature = "CloudKit_CKMarkNotificationsReadOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKMarkNotificationsReadOperation::CKMarkNotificationsReadOperation;
#[cfg(all(
    feature = "CloudKit_CKModifyBadgeOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKModifyBadgeOperation::CKModifyBadgeOperation;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKModifyRecordZonesOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKModifyRecordZonesOperation::CKModifyRecordZonesOperation;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKModifyRecordsOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKModifyRecordsOperation::CKModifyRecordsOperation;
#[cfg(feature = "CloudKit_CKModifyRecordsOperation")]
pub use self::__CKModifyRecordsOperation::CKRecordSavePolicy;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKModifySubscriptionsOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKModifySubscriptionsOperation::CKModifySubscriptionsOperation;
#[cfg(feature = "CloudKit_CKNotification")]
pub use self::__CKNotification::CKDatabaseNotification;
#[cfg(feature = "CloudKit_CKNotification")]
pub use self::__CKNotification::CKNotification;
#[cfg(feature = "CloudKit_CKNotification")]
pub use self::__CKNotification::CKNotificationID;
#[cfg(feature = "CloudKit_CKNotification")]
pub use self::__CKNotification::CKNotificationType;
#[cfg(feature = "CloudKit_CKNotification")]
pub use self::__CKNotification::CKQueryNotification;
#[cfg(feature = "CloudKit_CKNotification")]
pub use self::__CKNotification::CKQueryNotificationReason;
#[cfg(feature = "CloudKit_CKNotification")]
pub use self::__CKNotification::CKRecordZoneNotification;
#[cfg(all(feature = "CloudKit_CKOperation", feature = "Foundation_NSOperation"))]
pub use self::__CKOperation::CKOperation;
#[cfg(feature = "CloudKit_CKOperation")]
pub use self::__CKOperation::CKOperationConfiguration;
#[cfg(all(feature = "CloudKit_CKOperation", feature = "Foundation_NSString"))]
pub use self::__CKOperation::CKOperationID;
#[cfg(feature = "CloudKit_CKOperationGroup")]
pub use self::__CKOperationGroup::CKOperationGroup;
#[cfg(feature = "CloudKit_CKOperationGroup")]
pub use self::__CKOperationGroup::CKOperationGroupTransferSize;
#[cfg(feature = "CloudKit_CKQuery")]
pub use self::__CKQuery::CKQuery;
#[cfg(feature = "CloudKit_CKQueryOperation")]
pub use self::__CKQueryOperation::CKQueryCursor;
#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKOperation",
    feature = "CloudKit_CKQueryOperation",
    feature = "Foundation_NSOperation"
))]
pub use self::__CKQueryOperation::CKQueryOperation;
#[cfg(feature = "CloudKit_CKQueryOperation")]
pub use self::__CKQueryOperation::CKQueryOperationMaximumResults;
#[cfg(feature = "CloudKit_CKRecord")]
pub use self::__CKRecord::CKRecord;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
pub use self::__CKRecord::CKRecordCreationDateKey;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
pub use self::__CKRecord::CKRecordCreatorUserRecordIDKey;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
pub use self::__CKRecord::CKRecordFieldKey;
#[cfg(feature = "CloudKit_CKRecord")]
pub use self::__CKRecord::CKRecordKeyValueSetting;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
pub use self::__CKRecord::CKRecordLastModifiedUserRecordIDKey;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
pub use self::__CKRecord::CKRecordModificationDateKey;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
pub use self::__CKRecord::CKRecordParentKey;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
pub use self::__CKRecord::CKRecordRecordIDKey;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
pub use self::__CKRecord::CKRecordShareKey;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
pub use self::__CKRecord::CKRecordType;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
pub use self::__CKRecord::CKRecordTypeUserRecord;
#[cfg(feature = "CloudKit_CKRecord")]
pub use self::__CKRecord::CKRecordValue;
#[cfg(feature = "CloudKit_CKRecordID")]
pub use self::__CKRecordID::CKRecordID;
#[cfg(feature = "CloudKit_CKRecordZone")]
pub use self::__CKRecordZone::CKRecordZone;
#[cfg(feature = "CloudKit_CKRecordZone")]
pub use self::__CKRecordZone::CKRecordZoneCapabilities;
#[cfg(all(feature = "CloudKit_CKRecordZone", feature = "Foundation_NSString"))]
pub use self::__CKRecordZone::CKRecordZoneDefaultName;
#[cfg(feature = "CloudKit_CKRecordZoneID")]
pub use self::__CKRecordZoneID::CKRecordZoneID;
#[cfg(feature = "CloudKit_CKReference")]
pub use self::__CKReference::CKReference;
#[cfg(feature = "CloudKit_CKReference")]
pub use self::__CKReference::CKReferenceAction;
#[cfg(feature = "CloudKit_CKServerChangeToken")]
pub use self::__CKServerChangeToken::CKServerChangeToken;
#[cfg(all(feature = "CloudKit_CKShare", feature = "Foundation_NSString"))]
pub use self::__CKShare::CKRecordNameZoneWideShare;
#[cfg(all(
    feature = "CloudKit_CKRecord",
    feature = "CloudKit_CKShare",
    feature = "Foundation_NSString"
))]
pub use self::__CKShare::CKRecordTypeShare;
#[cfg(all(feature = "CloudKit_CKRecord", feature = "CloudKit_CKShare"))]
pub use self::__CKShare::CKShare;
#[cfg(all(
    feature = "CloudKit_CKRecord",
    feature = "CloudKit_CKShare",
    feature = "Foundation_NSString"
))]
pub use self::__CKShare::CKShareThumbnailImageDataKey;
#[cfg(all(
    feature = "CloudKit_CKRecord",
    feature = "CloudKit_CKShare",
    feature = "Foundation_NSString"
))]
pub use self::__CKShare::CKShareTitleKey;
#[cfg(all(
    feature = "CloudKit_CKRecord",
    feature = "CloudKit_CKShare",
    feature = "Foundation_NSString"
))]
pub use self::__CKShare::CKShareTypeKey;
#[cfg(feature = "CloudKit_CKShareMetadata")]
pub use self::__CKShareMetadata::CKShareMetadata;
#[cfg(feature = "CloudKit_CKShareParticipant")]
pub use self::__CKShareParticipant::CKShareParticipant;
#[cfg(feature = "CloudKit_CKShareParticipant")]
pub use self::__CKShareParticipant::CKShareParticipantAcceptanceStatus;
#[cfg(feature = "CloudKit_CKShareParticipant")]
pub use self::__CKShareParticipant::CKShareParticipantPermission;
#[cfg(feature = "CloudKit_CKShareParticipant")]
pub use self::__CKShareParticipant::CKShareParticipantRole;
#[cfg(feature = "CloudKit_CKShareParticipant")]
pub use self::__CKShareParticipant::CKShareParticipantType;
#[cfg(feature = "CloudKit_CKSubscription")]
pub use self::__CKSubscription::CKDatabaseSubscription;
#[cfg(feature = "CloudKit_CKSubscription")]
pub use self::__CKSubscription::CKNotificationInfo;
#[cfg(feature = "CloudKit_CKSubscription")]
pub use self::__CKSubscription::CKQuerySubscription;
#[cfg(feature = "CloudKit_CKSubscription")]
pub use self::__CKSubscription::CKQuerySubscriptionOptions;
#[cfg(feature = "CloudKit_CKSubscription")]
pub use self::__CKSubscription::CKRecordZoneSubscription;
#[cfg(feature = "CloudKit_CKSubscription")]
pub use self::__CKSubscription::CKSubscription;
#[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSString"))]
pub use self::__CKSubscription::CKSubscriptionID;
#[cfg(feature = "CloudKit_CKSubscription")]
pub use self::__CKSubscription::CKSubscriptionType;
#[cfg(feature = "CloudKit_CKSyncEngine")]
pub use self::__CKSyncEngine::CKSyncEngine;
#[cfg(feature = "CloudKit_CKSyncEngine")]
pub use self::__CKSyncEngine::CKSyncEngineDelegate;
#[cfg(feature = "CloudKit_CKSyncEngine")]
pub use self::__CKSyncEngine::CKSyncEngineFetchChangesContext;
#[cfg(feature = "CloudKit_CKSyncEngine")]
pub use self::__CKSyncEngine::CKSyncEngineFetchChangesOptions;
#[cfg(feature = "CloudKit_CKSyncEngine")]
pub use self::__CKSyncEngine::CKSyncEngineFetchChangesScope;
#[cfg(feature = "CloudKit_CKSyncEngine")]
pub use self::__CKSyncEngine::CKSyncEngineSendChangesContext;
#[cfg(feature = "CloudKit_CKSyncEngine")]
pub use self::__CKSyncEngine::CKSyncEngineSendChangesOptions;
#[cfg(feature = "CloudKit_CKSyncEngine")]
pub use self::__CKSyncEngine::CKSyncEngineSendChangesScope;
#[cfg(feature = "CloudKit_CKSyncEngine")]
pub use self::__CKSyncEngine::CKSyncEngineSyncReason;
#[cfg(feature = "CloudKit_CKSyncEngineConfiguration")]
pub use self::__CKSyncEngineConfiguration::CKSyncEngineConfiguration;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineAccountChangeEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineAccountChangeType;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineDidFetchChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineDidFetchRecordZoneChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineDidSendChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineEventType;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFailedRecordSave;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFailedZoneSave;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFetchedDatabaseChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFetchedRecordDeletion;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFetchedRecordZoneChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFetchedZoneDeletion;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineSentDatabaseChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineSentRecordZoneChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineStateUpdateEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineWillFetchChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineWillFetchRecordZoneChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineWillSendChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineZoneDeletionReason;
#[cfg(feature = "CloudKit_CKSyncEngineRecordZoneChangeBatch")]
pub use self::__CKSyncEngineRecordZoneChangeBatch::CKSyncEngineRecordZoneChangeBatch;
#[cfg(feature = "CloudKit_CKSyncEngineState")]
pub use self::__CKSyncEngineState::CKSyncEnginePendingDatabaseChange;
#[cfg(feature = "CloudKit_CKSyncEngineState")]
pub use self::__CKSyncEngineState::CKSyncEnginePendingDatabaseChangeType;
#[cfg(feature = "CloudKit_CKSyncEngineState")]
pub use self::__CKSyncEngineState::CKSyncEnginePendingRecordZoneChange;
#[cfg(feature = "CloudKit_CKSyncEngineState")]
pub use self::__CKSyncEngineState::CKSyncEnginePendingRecordZoneChangeType;
#[cfg(feature = "CloudKit_CKSyncEngineState")]
pub use self::__CKSyncEngineState::CKSyncEnginePendingZoneDelete;
#[cfg(feature = "CloudKit_CKSyncEngineState")]
pub use self::__CKSyncEngineState::CKSyncEnginePendingZoneSave;
#[cfg(feature = "CloudKit_CKSyncEngineState")]
pub use self::__CKSyncEngineState::CKSyncEngineState;
#[cfg(feature = "CloudKit_CKSyncEngineState")]
pub use self::__CKSyncEngineState::CKSyncEngineStateSerialization;
#[cfg(feature = "CloudKit_CKSystemSharingUIObserver")]
pub use self::__CKSystemSharingUIObserver::CKSystemSharingUIObserver;
#[cfg(feature = "CloudKit_CKUserIdentity")]
pub use self::__CKUserIdentity::CKUserIdentity;
#[cfg(feature = "CloudKit_CKUserIdentityLookupInfo")]
pub use self::__CKUserIdentityLookupInfo::CKUserIdentityLookupInfo;
#[cfg(all(
    feature = "CloudKit_CKRecord",
    feature = "CloudKit_CKShare",
    feature = "CloudKit_NSItemProvider_CKSharingSupport",
    feature = "Foundation_NSError"
))]
pub use self::__NSItemProvider_CKSharingSupport::CKSharePreparationCompletionHandler;
#[cfg(all(
    feature = "CloudKit_CKRecord",
    feature = "CloudKit_CKShare",
    feature = "CloudKit_NSItemProvider_CKSharingSupport",
    feature = "Foundation_NSError"
))]
pub use self::__NSItemProvider_CKSharingSupport::CKSharePreparationHandler;
#[cfg(feature = "CloudKit_NSItemProvider_CKSharingSupport")]
pub use self::__NSItemProvider_CKSharingSupport::NSItemProviderCKSharingSupport;
