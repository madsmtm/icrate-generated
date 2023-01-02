//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSSQLiteStoreType: &'static NSString);

extern_static!(NSXMLStoreType: &'static NSString);

extern_static!(NSBinaryStoreType: &'static NSString);

extern_static!(NSInMemoryStoreType: &'static NSString);

extern_static!(NSStoreTypeKey: &'static NSString);

extern_static!(NSStoreUUIDKey: &'static NSString);

extern_static!(NSPersistentStoreCoordinatorStoresWillChangeNotification: &'static NSString);

extern_static!(NSPersistentStoreCoordinatorStoresDidChangeNotification: &'static NSString);

extern_static!(NSPersistentStoreCoordinatorWillRemoveStoreNotification: &'static NSString);

extern_static!(NSAddedPersistentStoresKey: &'static NSString);

extern_static!(NSRemovedPersistentStoresKey: &'static NSString);

extern_static!(NSUUIDChangedPersistentStoresKey: &'static NSString);

extern_static!(NSReadOnlyPersistentStoreOption: &'static NSString);

extern_static!(NSValidateXMLStoreOption: &'static NSString);

extern_static!(NSPersistentStoreTimeoutOption: &'static NSString);

extern_static!(NSSQLitePragmasOption: &'static NSString);

extern_static!(NSSQLiteAnalyzeOption: &'static NSString);

extern_static!(NSSQLiteManualVacuumOption: &'static NSString);

extern_static!(NSIgnorePersistentStoreVersioningOption: &'static NSString);

extern_static!(NSMigratePersistentStoresAutomaticallyOption: &'static NSString);

extern_static!(NSInferMappingModelAutomaticallyOption: &'static NSString);

extern_static!(NSStoreModelVersionHashesKey: &'static NSString);

extern_static!(NSStoreModelVersionIdentifiersKey: &'static NSString);

extern_static!(NSPersistentStoreOSCompatibility: &'static NSString);

extern_static!(NSPersistentStoreConnectionPoolMaxSizeKey: &'static NSString);

extern_static!(NSCoreDataCoreSpotlightExporter: &'static NSString);

extern_static!(NSXMLExternalRecordType: &'static NSString);

extern_static!(NSBinaryExternalRecordType: &'static NSString);

extern_static!(NSExternalRecordsFileFormatOption: &'static NSString);

extern_static!(NSExternalRecordsDirectoryOption: &'static NSString);

extern_static!(NSExternalRecordExtensionOption: &'static NSString);

extern_static!(NSEntityNameInPathKey: &'static NSString);

extern_static!(NSStoreUUIDInPathKey: &'static NSString);

extern_static!(NSStorePathKey: &'static NSString);

extern_static!(NSModelPathKey: &'static NSString);

extern_static!(NSObjectURIKey: &'static NSString);

extern_static!(NSPersistentStoreForceDestroyOption: &'static NSString);

extern_static!(NSPersistentStoreFileProtectionKey: &'static NSString);

extern_static!(NSPersistentHistoryTrackingKey: &'static NSString);

extern_static!(NSBinaryStoreSecureDecodingClasses: &'static NSString);

extern_static!(NSBinaryStoreInsecureDecodingCompatibilityOption: &'static NSString);

extern_static!(NSPersistentStoreRemoteChangeNotificationPostOptionKey: &'static NSString);

extern_static!(NSPersistentStoreRemoteChangeNotification: &'static NSString);

extern_static!(NSPersistentStoreURLKey: &'static NSString);

extern_static!(NSPersistentHistoryTokenKey: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentStoreCoordinator;

    unsafe impl ClassType for NSPersistentStoreCoordinator {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPersistentStoreCoordinator {
        #[method_id(@__retain_semantics Init initWithManagedObjectModel:)]
        pub unsafe fn initWithManagedObjectModel(
            this: Option<Allocated<Self>>,
            model: &NSManagedObjectModel,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Id<NSManagedObjectModel, Shared>;

        #[method_id(@__retain_semantics Other persistentStores)]
        pub unsafe fn persistentStores(&self) -> Id<NSArray<NSPersistentStore>, Shared>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other persistentStoreForURL:)]
        pub unsafe fn persistentStoreForURL(
            &self,
            URL: &NSURL,
        ) -> Option<Id<NSPersistentStore, Shared>>;

        #[method_id(@__retain_semantics Other URLForPersistentStore:)]
        pub unsafe fn URLForPersistentStore(&self, store: &NSPersistentStore) -> Id<NSURL, Shared>;

        #[method(setURL:forPersistentStore:)]
        pub unsafe fn setURL_forPersistentStore(
            &self,
            url: &NSURL,
            store: &NSPersistentStore,
        ) -> bool;

        #[method_id(@__retain_semantics Other addPersistentStoreWithType:configuration:URL:options:error:_)]
        pub unsafe fn addPersistentStoreWithType_configuration_URL_options_error(
            &self,
            storeType: &NSString,
            configuration: Option<&NSString>,
            storeURL: Option<&NSURL>,
            options: Option<&NSDictionary>,
        ) -> Result<Id<NSPersistentStore, Shared>, Id<NSError, Shared>>;

        #[method(addPersistentStoreWithDescription:completionHandler:)]
        pub unsafe fn addPersistentStoreWithDescription_completionHandler(
            &self,
            storeDescription: &NSPersistentStoreDescription,
            block: &Block<(NonNull<NSPersistentStoreDescription>, *mut NSError), ()>,
        );

        #[method(removePersistentStore:error:_)]
        pub unsafe fn removePersistentStore_error(
            &self,
            store: &NSPersistentStore,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(setMetadata:forPersistentStore:)]
        pub unsafe fn setMetadata_forPersistentStore(
            &self,
            metadata: Option<&NSDictionary<NSString, Object>>,
            store: &NSPersistentStore,
        );

        #[method_id(@__retain_semantics Other metadataForPersistentStore:)]
        pub unsafe fn metadataForPersistentStore(
            &self,
            store: &NSPersistentStore,
        ) -> Id<NSDictionary<NSString, Object>, Shared>;

        #[method_id(@__retain_semantics Other managedObjectIDForURIRepresentation:)]
        pub unsafe fn managedObjectIDForURIRepresentation(
            &self,
            url: &NSURL,
        ) -> Option<Id<NSManagedObjectID, Shared>>;

        #[method_id(@__retain_semantics Other executeRequest:withContext:error:_)]
        pub unsafe fn executeRequest_withContext_error(
            &self,
            request: &NSPersistentStoreRequest,
            context: &NSManagedObjectContext,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other registeredStoreTypes)]
        pub unsafe fn registeredStoreTypes() -> Id<NSDictionary<NSString, NSValue>, Shared>;

        #[method(registerStoreClass:forStoreType:)]
        pub unsafe fn registerStoreClass_forStoreType(
            storeClass: Option<&Class>,
            storeType: &NSString,
        );

        #[method_id(@__retain_semantics Other metadataForPersistentStoreOfType:URL:options:error:_)]
        pub unsafe fn metadataForPersistentStoreOfType_URL_options_error(
            storeType: &NSString,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Result<Id<NSDictionary<NSString, Object>, Shared>, Id<NSError, Shared>>;

        #[method(setMetadata:forPersistentStoreOfType:URL:options:error:_)]
        pub unsafe fn setMetadata_forPersistentStoreOfType_URL_options_error(
            metadata: Option<&NSDictionary<NSString, Object>>,
            storeType: &NSString,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other elementsDerivedFromExternalRecordURL:)]
        pub unsafe fn elementsDerivedFromExternalRecordURL(
            fileURL: &NSURL,
        ) -> Id<NSDictionary, Shared>;

        #[method_id(@__retain_semantics Other importStoreWithIdentifier:fromExternalRecordsDirectory:toURL:options:withType:error:_)]
        pub unsafe fn importStoreWithIdentifier_fromExternalRecordsDirectory_toURL_options_withType_error(
            &self,
            storeIdentifier: Option<&NSString>,
            externalRecordsURL: &NSURL,
            destinationURL: &NSURL,
            options: Option<&NSDictionary>,
            storeType: &NSString,
        ) -> Result<Id<NSPersistentStore, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other migratePersistentStore:toURL:options:withType:error:_)]
        pub unsafe fn migratePersistentStore_toURL_options_withType_error(
            &self,
            store: &NSPersistentStore,
            URL: &NSURL,
            options: Option<&NSDictionary>,
            storeType: &NSString,
        ) -> Result<Id<NSPersistentStore, Shared>, Id<NSError, Shared>>;

        #[method(destroyPersistentStoreAtURL:withType:options:error:_)]
        pub unsafe fn destroyPersistentStoreAtURL_withType_options_error(
            &self,
            url: &NSURL,
            storeType: &NSString,
            options: Option<&NSDictionary>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(replacePersistentStoreAtURL:destinationOptions:withPersistentStoreFromURL:sourceOptions:storeType:error:_)]
        pub unsafe fn replacePersistentStoreAtURL_destinationOptions_withPersistentStoreFromURL_sourceOptions_storeType_error(
            &self,
            destinationURL: &NSURL,
            destinationOptions: Option<&NSDictionary>,
            sourceURL: &NSURL,
            sourceOptions: Option<&NSDictionary>,
            storeType: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(performBlock:)]
        pub unsafe fn performBlock(&self, block: &Block<(), ()>);

        #[method(performBlockAndWait:)]
        pub unsafe fn performBlockAndWait(&self, block: &Block<(), ()>);

        #[method_id(@__retain_semantics Other currentPersistentHistoryTokenFromStores:)]
        pub unsafe fn currentPersistentHistoryTokenFromStores(
            &self,
            stores: Option<&NSArray>,
        ) -> Option<Id<NSPersistentHistoryToken, Shared>>;

        #[method_id(@__retain_semantics Other metadataForPersistentStoreWithURL:error:_)]
        pub unsafe fn metadataForPersistentStoreWithURL_error(
            url: &NSURL,
        ) -> Result<Id<NSDictionary, Shared>, Id<NSError, Shared>>;

        #[method(lock)]
        pub unsafe fn lock(&self);

        #[method(unlock)]
        pub unsafe fn unlock(&self);

        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[method_id(@__retain_semantics Other metadataForPersistentStoreOfType:URL:error:_)]
        pub unsafe fn metadataForPersistentStoreOfType_URL_error(
            storeType: Option<&NSString>,
            url: &NSURL,
        ) -> Result<Id<NSDictionary<NSString, Object>, Shared>, Id<NSError, Shared>>;

        #[method(setMetadata:forPersistentStoreOfType:URL:error:_)]
        pub unsafe fn setMetadata_forPersistentStoreOfType_URL_error(
            metadata: Option<&NSDictionary<NSString, Object>>,
            storeType: Option<&NSString>,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(removeUbiquitousContentAndPersistentStoreAtURL:options:error:_)]
        pub unsafe fn removeUbiquitousContentAndPersistentStoreAtURL_options_error(
            storeURL: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSPersistentStoreUbiquitousTransitionType {
        NSPersistentStoreUbiquitousTransitionTypeAccountAdded = 1,
        NSPersistentStoreUbiquitousTransitionTypeAccountRemoved = 2,
        NSPersistentStoreUbiquitousTransitionTypeContentRemoved = 3,
        NSPersistentStoreUbiquitousTransitionTypeInitialImportCompleted = 4,
    }
);

extern_static!(NSPersistentStoreUbiquitousContentNameKey: &'static NSString);

extern_static!(NSPersistentStoreUbiquitousContentURLKey: &'static NSString);

extern_static!(NSPersistentStoreDidImportUbiquitousContentChangesNotification: &'static NSString);

extern_static!(NSPersistentStoreUbiquitousTransitionTypeKey: &'static NSString);

extern_static!(NSPersistentStoreUbiquitousPeerTokenOption: &'static NSString);

extern_static!(NSPersistentStoreRemoveUbiquitousMetadataOption: &'static NSString);

extern_static!(NSPersistentStoreUbiquitousContainerIdentifierKey: &'static NSString);

extern_static!(NSPersistentStoreRebuildFromUbiquitousContentOption: &'static NSString);
